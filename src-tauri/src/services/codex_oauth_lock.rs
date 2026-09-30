//! Codex OAuth account removal, serialized with provider switching.
//! Shared by the desktop auth commands and the provider service.

use crate::app_config::AppType;
use crate::store::AppState;

pub async fn remove_codex_oauth_account_with_switch_lock(
    app_state: &AppState,
    account_id: &str,
) -> Result<(), String> {
    let _switch_guard = app_state
        .proxy_service
        .lock_switch_for_app(AppType::Codex.as_str())
        .await;
    app_state
        .codex_oauth_manager
        .remove_account(account_id)
        .await
        .map_err(|error| error.to_string())
}

pub async fn logout_codex_oauth_with_switch_lock(app_state: &AppState) -> Result<(), String> {
    let _switch_guard = app_state
        .proxy_service
        .lock_switch_for_app(AppType::Codex.as_str())
        .await;
    app_state
        .codex_oauth_manager
        .clear_auth()
        .await
        .map_err(|error| error.to_string())
}
