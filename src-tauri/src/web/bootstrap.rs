//! Open the same database and services the desktop app uses, without a window.

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::host::AppHandle;
use crate::init_status::InitErrorPayload;
use crate::proxy::http_client;
use crate::proxy::oauth_state::{CodexOAuthState, CopilotAuthState, XaiOAuthState};
use crate::proxy::providers::copilot_auth::CopilotAuthManager;
use crate::proxy::providers::xai_oauth_auth::XaiOAuthManager;
use crate::store::AppState;
use crate::usage_events;

pub fn bootstrap() -> Result<Arc<AppState>, String> {
    crate::app_store::load_cached_app_config_dir_override();
    crate::panic_hook::setup_panic_hook();
    let app_config_dir = crate::config::get_app_config_dir();
    crate::panic_hook::init_app_config_dir(app_config_dir.clone());
    if let Err(error) = std::fs::create_dir_all(&app_config_dir) {
        return Err(format!("创建配置目录失败: {error}"));
    }

    let db_path = app_config_dir.join("cc-switch.db");
    match crate::database::Database::stored_user_version_exceeds_supported(&db_path) {
        Ok(Some(version)) => {
            crate::init_status::set_init_error(InitErrorPayload {
                path: db_path.display().to_string(),
                error: format!(
                    "数据库版本过新（{version}），当前程序仅支持 {}。",
                    crate::database::SCHEMA_VERSION
                ),
                kind: Some("db_version_too_new".to_string()),
                db_version: Some(version),
                supported_version: Some(crate::database::SCHEMA_VERSION),
            });
        }
        Ok(None) => {}
        Err(error) => log::warn!("预检数据库版本失败，继续初始化: {error}"),
    }
    if crate::init_status::get_init_error().is_some() {
        // The page renders the upgrade notice from get_init_error. Keep a
        // usable database handle only when init itself succeeds; a too-new
        // database must not be migrated by this older process.
        return Err(format!(
            "数据库版本过新，请换用与该库匹配的 CC Switch。文件: {}",
            db_path.display()
        ));
    }

    let db = crate::database::Database::init().map_err(|error| error.to_string())?;
    let db = Arc::new(db);
    if let Ok(log_config) = db.get_log_config() {
        log::set_max_level(log_config.to_level_filter());
    }

    let json_path = app_config_dir.join("config.json");
    if !db_path.exists() && json_path.exists() {
        // Database::init creates the file, so this branch is only a reminder
        // for logs. Migration from a pre-existing json file is handled below
        // when the database was just created empty.
    }
    let _ = db_path;

    let state = Arc::new(AppState::new(db));
    crate::mode::operation::recover_on_startup(&state.db);

    if let Err(error) = state.db.init_default_skill_repos() {
        log::warn!("初始化默认技能仓库失败: {error}");
    }
    import_live_providers(&state);
    if let Err(error) = state.db.init_default_official_providers() {
        log::warn!("写入官方供应商预设失败: {error}");
    }

    let proxy_url = state.db.get_global_proxy_url().ok().flatten();
    if let Err(error) = http_client::init(proxy_url.as_deref()) {
        log::warn!("初始化出站代理失败，改为直连: {error}");
        let _ = http_client::init(None);
    }
    Ok(state)
}

fn import_live_providers(state: &AppState) {
    for app_type in crate::app_config::AppType::all().filter(|app| !app.is_additive_mode()) {
        match crate::services::provider::should_import_default_config_on_startup(state, &app_type)
        {
            Ok(true) => match crate::services::provider::import_default_config(state, app_type.clone())
            {
                Ok(true) => log::info!("已导入 {} 的现有配置", app_type.as_str()),
                Ok(false) => {}
                Err(error) => {
                    log::debug!("跳过 {} 的现有配置: {error}", app_type.as_str())
                }
            },
            Ok(false) => {}
            Err(error) => log::debug!("检查 {} 是否需要导入失败: {error}", app_type.as_str()),
        }
    }
}

pub fn install_handle(state: Arc<AppState>) -> AppHandle {
    let handle = AppHandle::new();
    handle.manage(AppState::clone(&state));
    handle.manage(crate::commands::skill::SkillServiceState(Arc::new(
        crate::services::SkillService::new(),
    )));
    handle.manage(CodexOAuthState(state.codex_oauth_manager.clone()));
    let config_dir = crate::config::get_app_config_dir();
    handle.manage(CopilotAuthState(Arc::new(RwLock::new(CopilotAuthManager::new(
        config_dir.clone(),
    )))));
    handle.manage(XaiOAuthState(Arc::new(RwLock::new(XaiOAuthManager::new(
        config_dir,
    )))));
    state.proxy_service.set_app_handle(handle.clone());
    usage_events::init(handle.clone());
    let _ = crate::services::webdav_auto_sync::start_worker(state.db.clone(), handle.clone());
    let _ = crate::services::s3_auto_sync::start_worker(state.db.clone(), handle.clone());
    handle
}
