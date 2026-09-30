//! Local management server for the existing React UI.
//! Built with `--no-default-features --features web`. It does not open a window.

mod bootstrap;
mod dispatch;
mod generated;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

use crate::host::{install_runtime, AppHandle};
use crate::store::AppState;

struct WebState {
    app_state: Arc<AppState>,
    handle: AppHandle,
    token: String,
    dist: PathBuf,
}

pub fn serve() {
    let _ = rustls::crypto::ring::default_provider().install_default();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut host = "127.0.0.1".to_string();
    let mut port: u16 = 3927;
    let mut dist = default_dist_dir();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--host" => {
                host = args.next().unwrap_or_else(|| missing_arg("--host"));
            }
            "--port" => {
                let raw = args.next().unwrap_or_else(|| missing_arg("--port"));
                port = raw.parse().unwrap_or_else(|_| {
                    eprintln!("无效端口: {raw}");
                    std::process::exit(2);
                });
            }
            "--dist" => {
                dist = PathBuf::from(args.next().unwrap_or_else(|| missing_arg("--dist")));
            }
            "--help" | "-h" => {
                println!(
                    "用法: cc-switch-web [--host 127.0.0.1] [--port 3927] [--dist ./dist]\n\
                     默认只监听本机。在自己的电脑上执行 ssh -L 3927:127.0.0.1:3927 后打开 http://127.0.0.1:3927/\n\
                     访问令牌保存在 ~/.cc-switch/web-token，重启后不变。浏览器记住一次后不必再写在地址里。"
                );
                return;
            }
            other => {
                eprintln!("未知参数: {other}");
                std::process::exit(2);
            }
        }
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("创建 Tokio 运行时失败");
    install_runtime(runtime.handle().clone());

    let app_state = match bootstrap::bootstrap() {
        Ok(state) => state,
        Err(error) => {
            eprintln!("启动失败: {error}");
            std::process::exit(1);
        }
    };
    let handle = bootstrap::install_handle(app_state.clone());
    let token = load_or_create_web_token();
    let state = Arc::new(WebState {
        app_state,
        handle,
        token: token.clone(),
        dist: dist.clone(),
    });

    runtime.block_on(async move {
        let app = router(state.clone());
        let addr: SocketAddr = format!("{host}:{port}")
            .parse()
            .unwrap_or_else(|_| {
                eprintln!("无效监听地址: {host}:{port}");
                std::process::exit(2);
            });
        let listener = TcpListener::bind(addr).await.unwrap_or_else(|error| {
            eprintln!("绑定 {addr} 失败: {error}");
            std::process::exit(1);
        });
        let local = listener.local_addr().unwrap_or(addr);
        println!();
        println!("CC Switch 网页已启动");
        println!("  本机地址: http://{local}/");
        println!("  第一次打开: http://{local}/#token={token}");
        println!("  令牌文件: {}", web_token_path().display());
        println!("  页面目录: {}", dist.display());
        println!("  停止: Ctrl+C");
        println!("  从自己的电脑打开时，先把端口转过来，例如:");
        println!("    ssh -L {port}:127.0.0.1:{port} <user>@<server>");
        println!();
        axum::serve(listener, app)
            .await
            .expect("网页服务异常退出");
    });
}

fn missing_arg(flag: &str) -> String {
    eprintln!("缺少 {flag} 的值");
    std::process::exit(2);
}

fn web_token_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cc-switch")
        .join("web-token")
}

fn load_or_create_web_token() -> String {
    let path = web_token_path();
    if let Ok(existing) = std::fs::read_to_string(&path) {
        let token = existing.trim().to_string();
        if !token.is_empty() {
            return token;
        }
    }
    let token = uuid::Uuid::new_v4().simple().to_string();
    if let Some(parent) = path.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            eprintln!("无法创建 {}: {error}", parent.display());
            return token;
        }
    }
    if let Err(error) = std::fs::write(&path, format!("{token}\n")) {
        eprintln!("无法保存访问令牌到 {}: {error}", path.display());
        return token;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }
    token
}

fn default_dist_dir() -> PathBuf {
    if let Some(path) = std::env::var_os("CC_SWITCH_WEB_DIST") {
        return PathBuf::from(path);
    }
    let cwd = PathBuf::from("dist");
    if cwd.join("index.html").is_file() {
        return cwd;
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let beside = dir.join("dist");
            if beside.join("index.html").is_file() {
                return beside;
            }
        }
    }
    cwd
}

fn router(state: Arc<WebState>) -> Router {
    let index = state.dist.join("index.html");
    let files = ServeDir::new(&state.dist).not_found_service(ServeFile::new(index));
    Router::new()
        .route("/api/v1/bootstrap", get(bootstrap_info))
        .route("/api/v1/invoke/:command", post(invoke))
        .route("/api/v1/events", get(events))
        .fallback_service(files)
        .with_state(state)
}

async fn bootstrap_info(State(state): State<Arc<WebState>>, headers: HeaderMap) -> Response {
    if let Err(response) = authorize(&state, &headers, None) {
        return response;
    }
    Json(json!({ "mode": "web", "version": env!("CARGO_PKG_VERSION") })).into_response()
}

async fn invoke(
    State(state): State<Arc<WebState>>,
    headers: HeaderMap,
    Path(command): Path<String>,
    body: String,
) -> Response {
    if let Err(response) = authorize(&state, &headers, None) {
        return response;
    }
    let payload = if body.trim().is_empty() {
        Value::Object(Default::default())
    } else {
        match serde_json::from_str::<Value>(&body) {
            Ok(value) => value,
            Err(error) => {
                return (StatusCode::BAD_REQUEST, format!("请求不是 JSON: {error}")).into_response()
            }
        }
    };
    match dispatch::dispatch(&state.app_state, &state.handle, &command, payload).await {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::BAD_REQUEST, error).into_response(),
    }
}

#[derive(Deserialize)]
struct TokenQuery {
    token: String,
}

async fn events(
    State(state): State<Arc<WebState>>,
    headers: HeaderMap,
    Query(query): Query<TokenQuery>,
    upgrade: WebSocketUpgrade,
) -> Response {
    if let Err(response) = authorize(&state, &headers, Some(&query.token)) {
        return response;
    }
    let mut events = state.handle.subscribe();
    upgrade.on_upgrade(move |socket| async move {
        forward_events(socket, &mut events).await;
    })
}

async fn forward_events(
    mut socket: WebSocket,
    events: &mut tokio::sync::broadcast::Receiver<crate::host::WebEvent>,
) {
    loop {
        match events.recv().await {
            Ok(event) => {
                let text = serde_json::json!({
                    "event": event.name,
                    "payload": event.payload,
                })
                .to_string();
                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
        }
    }
}

fn authorize(
    state: &WebState,
    headers: &HeaderMap,
    query_token: Option<&str>,
) -> Result<(), Response> {
    let header_token = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim);
    let presented = header_token.or(query_token);
    if presented == Some(state.token.as_str()) {
        Ok(())
    } else {
        Err((StatusCode::UNAUTHORIZED, "未授权").into_response())
    }
}
