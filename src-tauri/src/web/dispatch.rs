//! Web invoke entry. Almost every desktop command is forwarded by `generated`.

use std::sync::Arc;

use serde_json::{json, Value};

use crate::host::AppHandle;
use crate::store::AppState;

use super::generated;

pub async fn dispatch(
    state: &Arc<AppState>,
    handle: &AppHandle,
    command: &str,
    payload: Value,
) -> Result<Value, String> {
    if let Some(value) = generated::try_dispatch(state, handle, command, payload.clone()).await? {
        if command == "switch_provider" {
            let _ = handle.emit(
                "provider-switched",
                json!({
                    "appType": payload.get("app").cloned().unwrap_or(Value::Null),
                    "providerId": payload.get("id").cloned().unwrap_or(Value::Null),
                }),
            );
        }
        return Ok(value);
    }

    match command {
        "set_window_theme" | "update_tray_menu" | "enter_lightweight_mode"
        | "exit_lightweight_mode" => Ok(json!(true)),
        "is_lightweight_mode" => Ok(json!(false)),
        other => Err(format!("网页模式还没有接上命令 {other}")),
    }
}
