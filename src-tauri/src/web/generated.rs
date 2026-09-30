//! Generated from desktop command signatures.
#![allow(non_snake_case, dead_code, clippy::all)]
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use crate::host::{AppHandle, State};
use crate::store::AppState;

pub async fn try_dispatch(
    _state: &Arc<AppState>,
    handle: &AppHandle,
    command: &str,
    payload: Value,
) -> Result<Option<Value>, String> {
    match command {
        "add_custom_endpoint" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            providerId: String,
            url: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::add_custom_endpoint(managed::<crate::store::AppState>(handle)?, args.app, args.providerId, args.url))?))
        }
        "add_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            provider: crate::provider::Provider,
            #[serde(default)]
            addToLive: Option<bool>,
            #[serde(default)]
            editorSave: Option<crate::services::provider::EditorSave>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::add_provider(handle.clone(), args.app, args.provider, args.addToLive, args.editorSave).await)?))
        }
        "add_skill_repo" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            repo: crate::services::skill::SkillRepo,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::add_skill_repo(args.repo, managed::<crate::store::AppState>(handle)?))?))
        }
        "add_to_failover_queue" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            provider_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::add_to_failover_queue(managed::<crate::store::AppState>(handle)?, args.app_type, args.provider_id).await)?))
        }
        "apply_claude_onboarding_skip" => {
            Ok(Some(finish_result(crate::commands::apply_claude_onboarding_skip().await)?))
        }
        "apply_claude_plugin_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            official: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::apply_claude_plugin_config(args.official).await)?))
        }
        "apply_profile" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            scope: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::apply_profile(handle.clone(), managed::<crate::store::AppState>(handle)?, args.id, args.scope))?))
        }
        "auth_cancel_login" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            device_code: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_cancel_login(args.auth_provider, args.device_code, managed::<crate::commands::codex_oauth::CodexOAuthState>(handle)?).await)?))
        }
        "auth_get_status" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_get_status(args.auth_provider, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::codex_oauth::CodexOAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?).await)?))
        }
        "auth_list_accounts" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_list_accounts(args.auth_provider, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::codex_oauth::CodexOAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?).await)?))
        }
        "auth_logout" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_logout(args.auth_provider, managed::<crate::store::AppState>(handle)?, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?).await)?))
        }
        "auth_poll_for_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            device_code: String,
            #[serde(default)]
            github_domain: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_poll_for_account(args.auth_provider, args.device_code, args.github_domain, managed::<crate::store::AppState>(handle)?, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::codex_oauth::CodexOAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?).await)?))
        }
        "auth_remove_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            account_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_remove_account(args.auth_provider, args.account_id, managed::<crate::store::AppState>(handle)?, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?).await)?))
        }
        "auth_set_default_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            account_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_set_default_account(args.auth_provider, args.account_id, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::codex_oauth::CodexOAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?).await)?))
        }
        "auth_start_login" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            auth_provider: String,
            #[serde(default)]
            github_domain: Option<String>,
            #[serde(default)]
            target_account_id: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::auth_start_login(args.auth_provider, args.github_domain, args.target_account_id, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::codex_oauth::CodexOAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?).await)?))
        }
        "check_app_update_available" => {
            Ok(Some(finish_result(crate::commands::check_app_update_available(handle.clone()).await)?))
        }
        "check_env_conflicts" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::check_env_conflicts(args.app))?))
        }
        "check_for_updates" => {
            Ok(Some(finish_result(crate::commands::check_for_updates(handle.clone()).await)?))
        }
        "check_provider_limits" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            provider_id: String,
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::check_provider_limits(managed::<crate::store::AppState>(handle)?, args.provider_id, args.app_type))?))
        }
        "check_skill_updates" => {
            Ok(Some(finish_result(crate::commands::check_skill_updates(managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "clear_claude_onboarding_skip" => {
            Ok(Some(finish_result(crate::commands::clear_claude_onboarding_skip().await)?))
        }
        "clear_current_profile" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            scope: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::clear_current_profile(managed::<crate::store::AppState>(handle)?, args.scope))?))
        }
        "copilot_get_auth_status" => {
            Ok(Some(finish_result(crate::commands::copilot_get_auth_status(managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_get_models" => {
            Ok(Some(finish_result(crate::commands::copilot_get_models(managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_get_models_for_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            account_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_get_models_for_account(args.account_id, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_get_token" => {
            Ok(Some(finish_result(crate::commands::copilot_get_token(managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_get_token_for_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            account_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_get_token_for_account(args.account_id, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_get_usage" => {
            Ok(Some(finish_result(crate::commands::copilot_get_usage(managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_get_usage_for_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            account_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_get_usage_for_account(args.account_id, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_is_authenticated" => {
            Ok(Some(finish_result(crate::commands::copilot_is_authenticated(managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_list_accounts" => {
            Ok(Some(finish_result(crate::commands::copilot_list_accounts(managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_logout" => {
            Ok(Some(finish_result(crate::commands::copilot_logout(managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_poll_for_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            device_code: String,
            #[serde(default)]
            github_domain: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_poll_for_account(args.device_code, args.github_domain, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_poll_for_auth" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            device_code: String,
            #[serde(default)]
            github_domain: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_poll_for_auth(args.device_code, args.github_domain, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_remove_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            account_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_remove_account(args.account_id, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_set_default_account" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            account_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_set_default_account(args.account_id, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copilot_start_device_flow" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            github_domain: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copilot_start_device_flow(args.github_domain, managed::<crate::proxy::oauth_state::CopilotAuthState>(handle)?).await)?))
        }
        "copy_text_to_clipboard" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            text: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::copy_text_to_clipboard(args.text).await)?))
        }
        "create_db_backup" => {
            Ok(Some(finish_result(crate::commands::create_db_backup(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "create_profile" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            name: String,
            scope: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::create_profile(managed::<crate::store::AppState>(handle)?, args.name, args.scope))?))
        }
        "delete_claude_mcp_server" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_claude_mcp_server(args.id).await)?))
        }
        "delete_daily_memory_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filename: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_daily_memory_file(args.filename).await)?))
        }
        "delete_db_backup" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filename: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_db_backup(args.filename))?))
        }
        "delete_env_vars" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            conflicts: Vec<crate::services::env_checker::EnvConflict>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_env_vars(args.conflicts))?))
        }
        "delete_mcp_server" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_mcp_server(managed::<crate::store::AppState>(handle)?, args.id).await)?))
        }
        "delete_mcp_server_in_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            _app: String,
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_mcp_server_in_config(managed::<crate::store::AppState>(handle)?, args._app, args.id).await)?))
        }
        "delete_model_pricing" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            model_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_model_pricing(managed::<crate::store::AppState>(handle)?, args.model_id))?))
        }
        "delete_pi_prompt_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            kind: crate::services::pi_prompt_files::PiPromptFileKind,
            expectedRevision: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_pi_prompt_file(args.kind, args.expectedRevision).await)?))
        }
        "delete_pi_prompt_template" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            slug: String,
            expectedRevision: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_pi_prompt_template(args.slug, args.expectedRevision).await)?))
        }
        "delete_profile" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_profile(managed::<crate::store::AppState>(handle)?, args.id))?))
        }
        "delete_prompt" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_prompt(args.app, args.id, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "delete_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_provider(managed::<crate::store::AppState>(handle)?, args.app, args.id))?))
        }
        "delete_session" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            providerId: String,
            sessionId: String,
            sourcePath: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_session(args.providerId, args.sessionId, args.sourcePath).await)?))
        }
        "delete_sessions" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            items: Vec<crate::session_manager::DeleteSessionRequest>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_sessions(args.items).await)?))
        }
        "delete_skill_backup" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            backup_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_skill_backup(args.backup_id))?))
        }
        "delete_universal_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::delete_universal_provider(handle.clone(), managed::<crate::store::AppState>(handle)?, args.id))?))
        }
        "disable_current_omo" => {
            Ok(Some(finish_result(crate::commands::disable_current_omo(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "disable_current_omo_slim" => {
            Ok(Some(finish_result(crate::commands::disable_current_omo_slim(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "discover_available_skills" => {
            Ok(Some(finish_result(crate::commands::discover_available_skills(managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "enable_prompt" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::enable_prompt(args.app, args.id, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "ensure_claude_desktop_official_provider" => {
            Ok(Some(finish_result(crate::commands::ensure_claude_desktop_official_provider(managed::<crate::store::AppState>(handle)?))?))
        }
        "ensure_codex_official_provider" => {
            Ok(Some(finish_result(crate::commands::ensure_codex_official_provider(managed::<crate::store::AppState>(handle)?))?))
        }
        "ensure_grokbuild_official_provider" => {
            Ok(Some(finish_result(crate::commands::ensure_grokbuild_official_provider(managed::<crate::store::AppState>(handle)?))?))
        }
        "enter_lightweight_mode" => {
            Ok(Some(finish_result(crate::commands::enter_lightweight_mode(handle.clone()))?))
        }
        "exit_lightweight_mode" => {
            Ok(Some(finish_result(crate::commands::exit_lightweight_mode(handle.clone()))?))
        }
        "export_config_to_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filePath: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::export_config_to_file(args.filePath, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "extract_common_config_snippet" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            appType: String,
            #[serde(default)]
            settingsConfig: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::extract_common_config_snippet(args.appType, args.settingsConfig, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "fetch_models_for_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            base_url: String,
            api_key: String,
            #[serde(default)]
            is_full_url: Option<bool>,
            #[serde(default)]
            models_url: Option<String>,
            #[serde(default)]
            custom_user_agent: Option<String>,
            #[serde(default)]
            api_format: Option<String>,
            #[serde(default)]
            request_headers: Option<std::collections::BTreeMap<String, String>>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::fetch_models_for_config(args.base_url, args.api_key, args.is_full_url, args.models_url, args.custom_user_agent, args.api_format, args.request_headers).await)?))
        }
        "get_app_config_dir_override" => {
            Ok(Some(finish_result(crate::commands::get_app_config_dir_override(handle.clone()).await)?))
        }
        "get_app_config_path" => {
            Ok(Some(finish_result(crate::commands::get_app_config_path().await)?))
        }
        "get_auto_failover_enabled" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_auto_failover_enabled(managed::<crate::store::AppState>(handle)?, args.app_type).await)?))
        }
        "get_auto_launch_status" => {
            Ok(Some(finish_result(crate::commands::get_auto_launch_status().await)?))
        }
        "get_available_providers_for_failover" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_available_providers_for_failover(managed::<crate::store::AppState>(handle)?, args.app_type).await)?))
        }
        "get_balance" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            base_url: String,
            api_key: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_balance(args.base_url, args.api_key).await)?))
        }
        "get_circuit_breaker_config" => {
            Ok(Some(finish_result(crate::commands::get_circuit_breaker_config(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_circuit_breaker_stats" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            provider_id: String,
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_circuit_breaker_stats(managed::<crate::store::AppState>(handle)?, args.provider_id, args.app_type).await)?))
        }
        "get_claude_code_config_path" => {
            Ok(Some(finish_result(crate::commands::get_claude_code_config_path().await)?))
        }
        "get_claude_common_config_snippet" => {
            Ok(Some(finish_result(crate::commands::get_claude_common_config_snippet(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_claude_config_status" => {
            Ok(Some(finish_result(crate::commands::get_claude_config_status().await)?))
        }
        "get_claude_desktop_default_routes" => {
            Ok(Some(finish(crate::commands::get_claude_desktop_default_routes())?))
        }
        "get_claude_desktop_status" => {
            Ok(Some(finish_result(crate::commands::get_claude_desktop_status(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_claude_mcp_status" => {
            Ok(Some(finish_result(crate::commands::get_claude_mcp_status().await)?))
        }
        "get_claude_plugin_status" => {
            Ok(Some(finish_result(crate::commands::get_claude_plugin_status().await)?))
        }
        "get_codex_oauth_models" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            account_id: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_codex_oauth_models(args.account_id, managed::<crate::proxy::oauth_state::CodexOAuthState>(handle)?).await)?))
        }
        "get_codex_oauth_quota" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            account_id: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_codex_oauth_quota(handle.clone(), managed::<crate::store::AppState>(handle)?, args.account_id, managed::<crate::proxy::oauth_state::CodexOAuthState>(handle)?).await)?))
        }
        "get_coding_plan_quota" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            base_url: String,
            api_key: String,
            #[serde(default)]
            access_key_id: Option<String>,
            #[serde(default)]
            secret_access_key: Option<String>,
            #[serde(default)]
            coding_plan_provider: Option<String>,
            #[serde(default)]
            team_organization_id: Option<String>,
            #[serde(default)]
            team_project_id: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_coding_plan_quota(args.base_url, args.api_key, args.access_key_id, args.secret_access_key, args.coding_plan_provider, args.team_organization_id, args.team_project_id).await)?))
        }
        "get_common_config_snippet" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_common_config_snippet(args.app_type, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_config_dir" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_config_dir(args.app).await)?))
        }
        "get_config_status" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_config_status(managed::<crate::store::AppState>(handle)?, args.app).await)?))
        }
        "get_copilot_optimizer_config" => {
            Ok(Some(finish_result(crate::commands::get_copilot_optimizer_config(managed::<crate::AppState>(handle)?).await)?))
        }
        "get_current_omo_provider_id" => {
            Ok(Some(finish_result(crate::commands::get_current_omo_provider_id(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_current_omo_slim_provider_id" => {
            Ok(Some(finish_result(crate::commands::get_current_omo_slim_provider_id(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_current_prompt_file_content" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_current_prompt_file_content(args.app).await)?))
        }
        "get_current_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_current_provider(managed::<crate::store::AppState>(handle)?, args.app))?))
        }
        "get_custom_endpoints" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            providerId: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_custom_endpoints(managed::<crate::store::AppState>(handle)?, args.app, args.providerId))?))
        }
        "get_direct_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_direct_provider(managed::<crate::store::AppState>(handle)?, args.app_type))?))
        }
        "get_failover_queue" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_failover_queue(managed::<crate::store::AppState>(handle)?, args.app_type).await)?))
        }
        "get_global_proxy_config" => {
            Ok(Some(finish_result(crate::commands::get_global_proxy_config(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_global_proxy_url" => {
            Ok(Some(finish_result(crate::commands::get_global_proxy_url(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_hermes_live_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            providerId: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_hermes_live_provider(args.providerId))?))
        }
        "get_hermes_live_provider_ids" => {
            Ok(Some(finish_result(crate::commands::get_hermes_live_provider_ids())?))
        }
        "get_hermes_memory" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            kind: crate::hermes_config::MemoryKind,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_hermes_memory(args.kind))?))
        }
        "get_hermes_memory_limits" => {
            Ok(Some(finish_result(crate::commands::get_hermes_memory_limits())?))
        }
        "get_hermes_model_config" => {
            Ok(Some(finish_result(crate::commands::get_hermes_model_config())?))
        }
        "get_init_error" => {
            Ok(Some(finish_result(crate::commands::get_init_error().await)?))
        }
        "get_installed_skills" => {
            Ok(Some(finish_result(crate::commands::get_installed_skills(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_log_config" => {
            Ok(Some(finish_result(crate::commands::get_log_config(managed::<crate::AppState>(handle)?).await)?))
        }
        "get_mcp_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_mcp_config(managed::<crate::store::AppState>(handle)?, args.app).await)?))
        }
        "get_mcp_servers" => {
            Ok(Some(finish_result(crate::commands::get_mcp_servers(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_migration_result" => {
            Ok(Some(finish_result(crate::commands::get_migration_result().await)?))
        }
        "get_model_pricing" => {
            Ok(Some(finish_result(crate::commands::get_model_pricing(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_model_stats" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            start_date: Option<i64>,
            #[serde(default)]
            end_date: Option<i64>,
            #[serde(default)]
            app_type: Option<String>,
            #[serde(default)]
            provider_name: Option<String>,
            #[serde(default)]
            model: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_model_stats(managed::<crate::store::AppState>(handle)?, args.start_date, args.end_date, args.app_type, args.provider_name, args.model))?))
        }
        "get_models_dev_sync_config" => {
            Ok(Some(finish_result(crate::commands::get_models_dev_sync_config(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_openclaw_agents_defaults" => {
            Ok(Some(finish_result(crate::commands::get_openclaw_agents_defaults())?))
        }
        "get_openclaw_default_model" => {
            Ok(Some(finish_result(crate::commands::get_openclaw_default_model())?))
        }
        "get_openclaw_env" => {
            Ok(Some(finish_result(crate::commands::get_openclaw_env())?))
        }
        "get_openclaw_live_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            providerId: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_openclaw_live_provider(args.providerId))?))
        }
        "get_openclaw_live_provider_ids" => {
            Ok(Some(finish_result(crate::commands::get_openclaw_live_provider_ids())?))
        }
        "get_openclaw_model_catalog" => {
            Ok(Some(finish_result(crate::commands::get_openclaw_model_catalog())?))
        }
        "get_openclaw_tools" => {
            Ok(Some(finish_result(crate::commands::get_openclaw_tools())?))
        }
        "get_opencode_live_provider_ids" => {
            Ok(Some(finish_result(crate::commands::get_opencode_live_provider_ids())?))
        }
        "get_opencode_models" => {
            Ok(Some(finish_result(crate::commands::get_opencode_models().await)?))
        }
        "get_optimizer_config" => {
            Ok(Some(finish_result(crate::commands::get_optimizer_config(managed::<crate::AppState>(handle)?).await)?))
        }
        "get_pi_current_state" => {
            Ok(Some(finish_result(crate::commands::get_pi_current_state(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_pi_prompt_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            kind: crate::services::pi_prompt_files::PiPromptFileKind,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_pi_prompt_file(args.kind).await)?))
        }
        "get_pi_session_discovery" => {
            Ok(Some(finish(crate::commands::get_pi_session_discovery())?))
        }
        "get_pricing_model_source" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_pricing_model_source(managed::<crate::store::AppState>(handle)?, args.app_type).await)?))
        }
        "get_prompts" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_prompts(args.app, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_provider_editor_view" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            settingsConfig: serde_json::Value,
            #[serde(default)]
            category: Option<String>,
            #[serde(default)]
            providerId: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_provider_editor_view(handle.clone(), args.app, args.settingsConfig, args.category, args.providerId).await)?))
        }
        "get_provider_health" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            provider_id: String,
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_provider_health(managed::<crate::store::AppState>(handle)?, args.provider_id, args.app_type).await)?))
        }
        "get_provider_stats" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            start_date: Option<i64>,
            #[serde(default)]
            end_date: Option<i64>,
            #[serde(default)]
            app_type: Option<String>,
            #[serde(default)]
            provider_name: Option<String>,
            #[serde(default)]
            model: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_provider_stats(managed::<crate::store::AppState>(handle)?, args.start_date, args.end_date, args.app_type, args.provider_name, args.model))?))
        }
        "get_providers" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_providers(managed::<crate::store::AppState>(handle)?, args.app))?))
        }
        "get_proxy_config" => {
            Ok(Some(finish_result(crate::commands::get_proxy_config(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_proxy_config_for_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_proxy_config_for_app(managed::<crate::store::AppState>(handle)?, args.app_type).await)?))
        }
        "get_proxy_status" => {
            Ok(Some(finish_result(crate::commands::get_proxy_status(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_proxy_takeover_status" => {
            Ok(Some(finish_result(crate::commands::get_proxy_takeover_status(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_rectifier_config" => {
            Ok(Some(finish_result(crate::commands::get_rectifier_config(managed::<crate::AppState>(handle)?).await)?))
        }
        "get_request_detail" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            request_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_request_detail(managed::<crate::store::AppState>(handle)?, args.request_id))?))
        }
        "get_request_logs" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filters: crate::services::usage_stats::LogFilters,
            page: u32,
            page_size: u32,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_request_logs(managed::<crate::store::AppState>(handle)?, args.filters, args.page, args.page_size))?))
        }
        "get_session_messages" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            providerId: String,
            sourcePath: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_session_messages(args.providerId, args.sourcePath).await)?))
        }
        "get_settings" => {
            Ok(Some(finish_result(crate::commands::get_settings().await)?))
        }
        "get_skill_backups" => {
            Ok(Some(finish_result(crate::commands::get_skill_backups())?))
        }
        "get_skill_repos" => {
            Ok(Some(finish_result(crate::commands::get_skill_repos(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_skills" => {
            Ok(Some(finish_result(crate::commands::get_skills(managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_skills_for_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_skills_for_app(args.app, managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "get_skills_migration_result" => {
            Ok(Some(finish_result(crate::commands::get_skills_migration_result().await)?))
        }
        "get_subscription_quota" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            tool: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_subscription_quota(handle.clone(), managed::<crate::store::AppState>(handle)?, args.tool).await)?))
        }
        "get_tool_versions" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            tools: Option<Vec<String>>,
            #[serde(default)]
            wsl_shell_by_tool: Option<std::collections::HashMap<String, crate::commands::misc::WslShellPreferenceInput>>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_tool_versions(args.tools, args.wsl_shell_by_tool).await)?))
        }
        "get_universal_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_universal_provider(managed::<crate::store::AppState>(handle)?, args.id))?))
        }
        "get_universal_providers" => {
            Ok(Some(finish_result(crate::commands::get_universal_providers(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_upstream_proxy_status" => {
            Ok(Some(finish(crate::commands::get_upstream_proxy_status())?))
        }
        "get_usage_data_sources" => {
            Ok(Some(finish_result(crate::commands::get_usage_data_sources(managed::<crate::store::AppState>(handle)?))?))
        }
        "get_usage_summary" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            start_date: Option<i64>,
            #[serde(default)]
            end_date: Option<i64>,
            #[serde(default)]
            app_type: Option<String>,
            #[serde(default)]
            provider_name: Option<String>,
            #[serde(default)]
            model: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_usage_summary(managed::<crate::store::AppState>(handle)?, args.start_date, args.end_date, args.app_type, args.provider_name, args.model))?))
        }
        "get_usage_summary_by_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            start_date: Option<i64>,
            #[serde(default)]
            end_date: Option<i64>,
            #[serde(default)]
            provider_name: Option<String>,
            #[serde(default)]
            model: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_usage_summary_by_app(managed::<crate::store::AppState>(handle)?, args.start_date, args.end_date, args.provider_name, args.model))?))
        }
        "get_usage_trends" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            start_date: Option<i64>,
            #[serde(default)]
            end_date: Option<i64>,
            #[serde(default)]
            app_type: Option<String>,
            #[serde(default)]
            provider_name: Option<String>,
            #[serde(default)]
            model: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_usage_trends(managed::<crate::store::AppState>(handle)?, args.start_date, args.end_date, args.app_type, args.provider_name, args.model))?))
        }
        "get_xai_oauth_models" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            account_id: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_xai_oauth_models(args.account_id, managed::<crate::proxy::oauth_state::XaiOAuthState>(handle)?).await)?))
        }
        "get_xai_oauth_quota" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            account_id: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::get_xai_oauth_quota(args.account_id, managed::<crate::proxy::oauth_state::XaiOAuthState>(handle)?).await)?))
        }
        "has_codex_unify_history_backup" => {
            Ok(Some(finish_result(crate::commands::has_codex_unify_history_backup().await)?))
        }
        "import_claude_desktop_providers_from_claude" => {
            Ok(Some(finish_result(crate::commands::import_claude_desktop_providers_from_claude(managed::<crate::store::AppState>(handle)?))?))
        }
        "import_config_from_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filePath: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::import_config_from_file(args.filePath, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "import_default_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::import_default_config(managed::<crate::store::AppState>(handle)?, args.app))?))
        }
        "import_from_deeplink" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            request: crate::deeplink::DeepLinkImportRequest,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::import_from_deeplink(managed::<crate::store::AppState>(handle)?, args.request))?))
        }
        "import_from_deeplink_unified" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            request: crate::deeplink::DeepLinkImportRequest,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::import_from_deeplink_unified(managed::<crate::store::AppState>(handle)?, args.request).await)?))
        }
        "import_hermes_providers_from_live" => {
            Ok(Some(finish_result(crate::commands::import_hermes_providers_from_live(managed::<crate::store::AppState>(handle)?))?))
        }
        "import_mcp_from_apps" => {
            Ok(Some(finish_result(crate::commands::import_mcp_from_apps(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "import_openclaw_providers_from_live" => {
            Ok(Some(finish_result(crate::commands::import_openclaw_providers_from_live(managed::<crate::store::AppState>(handle)?))?))
        }
        "import_opencode_providers_from_live" => {
            Ok(Some(finish_result(crate::commands::import_opencode_providers_from_live(managed::<crate::store::AppState>(handle)?))?))
        }
        "import_prompt_from_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::import_prompt_from_file(args.app, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "import_skills_from_apps" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            imports: Vec<crate::services::skill::ImportSkillSelection>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::import_skills_from_apps(args.imports, managed::<crate::store::AppState>(handle)?))?))
        }
        "install_skill" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            directory: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::install_skill(args.directory, managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "install_skill_for_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            directory: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::install_skill_for_app(args.app, args.directory, managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "install_skill_unified" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            skill: crate::services::skill::DiscoverableSkill,
            current_app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::install_skill_unified(args.skill, args.current_app, managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "install_skills_from_zip" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            file_path: String,
            current_app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::install_skills_from_zip(args.file_path, args.current_app, managed::<crate::store::AppState>(handle)?))?))
        }
        "install_update_and_restart" => {
            Ok(Some(finish_result(crate::commands::install_update_and_restart(handle.clone()).await)?))
        }
        "is_claude_plugin_applied" => {
            Ok(Some(finish_result(crate::commands::is_claude_plugin_applied().await)?))
        }
        "is_lightweight_mode" => {
            Ok(Some(finish(crate::commands::is_lightweight_mode())?))
        }
        "is_live_takeover_active" => {
            Ok(Some(finish_result(crate::commands::is_live_takeover_active(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "is_portable_mode" => {
            Ok(Some(finish_result(crate::commands::is_portable_mode().await)?))
        }
        "is_proxy_running" => {
            Ok(Some(finish_result(crate::commands::is_proxy_running(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "launch_hermes_dashboard" => {
            Ok(Some(finish_result(crate::commands::launch_hermes_dashboard().await)?))
        }
        "launch_session_terminal" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            command: String,
            #[serde(default)]
            cwd: Option<String>,
            #[serde(default)]
            custom_config: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::launch_session_terminal(args.command, args.cwd, args.custom_config).await)?))
        }
        "list_daily_memory_files" => {
            Ok(Some(finish_result(crate::commands::list_daily_memory_files().await)?))
        }
        "list_db_backups" => {
            Ok(Some(finish_result(crate::commands::list_db_backups())?))
        }
        "list_pi_prompt_templates" => {
            Ok(Some(finish_result(crate::commands::list_pi_prompt_templates().await)?))
        }
        "list_profiles" => {
            Ok(Some(finish_result(crate::commands::list_profiles(managed::<crate::store::AppState>(handle)?))?))
        }
        "list_sessions" => {
            Ok(Some(finish_result(crate::commands::list_sessions().await)?))
        }
        "merge_deeplink_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            request: crate::deeplink::DeepLinkImportRequest,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::merge_deeplink_config(args.request))?))
        }
        "migrate_skill_storage" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            target: crate::services::skill::SkillStorageLocation,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::migrate_skill_storage(args.target, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "open_app_config_folder" => {
            Ok(Some(finish_result(crate::commands::open_app_config_folder(handle.clone()).await)?))
        }
        "open_config_folder" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::open_config_folder(handle.clone(), args.app).await)?))
        }
        "open_external" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            url: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::open_external(handle.clone(), args.url).await)?))
        }
        "open_hermes_web_ui" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            path: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::open_hermes_web_ui(handle.clone(), args.path).await)?))
        }
        "open_provider_terminal" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            providerId: String,
            #[serde(default)]
            cwd: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::open_provider_terminal(managed::<crate::store::AppState>(handle)?, args.app, args.providerId, args.cwd).await)?))
        }
        "open_workspace_directory" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            subdir: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::open_workspace_directory(handle.clone(), args.subdir).await)?))
        }
        "parse_deeplink" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            url: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::parse_deeplink(args.url))?))
        }
        "pick_directory" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            defaultPath: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::pick_directory(handle.clone(), args.defaultPath).await)?))
        }
        "probe_tool_installations" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            tools: Vec<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::probe_tool_installations(args.tools).await)?))
        }
        "queryProviderUsage" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            providerId: String,
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::queryProviderUsage(handle.clone(), managed::<crate::store::AppState>(handle)?, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, managed::<crate::commands::xai_oauth::XaiOAuthState>(handle)?, args.providerId, args.app).await)?))
        }
        "read_claude_mcp_config" => {
            Ok(Some(finish_result(crate::commands::read_claude_mcp_config().await)?))
        }
        "read_claude_plugin_config" => {
            Ok(Some(finish_result(crate::commands::read_claude_plugin_config().await)?))
        }
        "read_daily_memory_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filename: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::read_daily_memory_file(args.filename).await)?))
        }
        "read_live_provider_settings" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::read_live_provider_settings(args.app))?))
        }
        "read_omo_local_file" => {
            Ok(Some(finish_result(crate::commands::read_omo_local_file().await)?))
        }
        "read_omo_slim_local_file" => {
            Ok(Some(finish_result(crate::commands::read_omo_slim_local_file().await)?))
        }
        "read_workspace_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filename: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::read_workspace_file(args.filename).await)?))
        }
        "rebuild_codex_usage" => {
            Ok(Some(finish_result(crate::commands::rebuild_codex_usage(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "record_models_dev_sync_result" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            synced_at: Option<i64>,
            #[serde(default)]
            error: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::record_models_dev_sync_result(managed::<crate::store::AppState>(handle)?, args.synced_at, args.error))?))
        }
        "remove_custom_endpoint" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            providerId: String,
            url: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::remove_custom_endpoint(managed::<crate::store::AppState>(handle)?, args.app, args.providerId, args.url))?))
        }
        "remove_from_failover_queue" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            provider_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::remove_from_failover_queue(managed::<crate::store::AppState>(handle)?, args.app_type, args.provider_id).await)?))
        }
        "remove_provider_from_live_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::remove_provider_from_live_config(managed::<crate::store::AppState>(handle)?, args.app, args.id))?))
        }
        "remove_skill_repo" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            owner: String,
            name: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::remove_skill_repo(args.owner, args.name, managed::<crate::store::AppState>(handle)?))?))
        }
        "rename_db_backup" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            oldFilename: String,
            newName: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::rename_db_backup(args.oldFilename, args.newName))?))
        }
        "replace_pi_prompt_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            kind: crate::services::pi_prompt_files::PiPromptFileKind,
            expectedRevision: String,
            content: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::replace_pi_prompt_file(args.kind, args.expectedRevision, args.content).await)?))
        }
        "reset_circuit_breaker" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            provider_id: String,
            app_type: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::reset_circuit_breaker(handle.clone(), managed::<crate::store::AppState>(handle)?, args.provider_id, args.app_type).await)?))
        }
        "restart_app" => {
            Ok(Some(finish_result(crate::commands::restart_app(handle.clone()).await)?))
        }
        "restore_codex_unified_history" => {
            Ok(Some(finish_result(crate::commands::restore_codex_unified_history().await)?))
        }
        "restore_db_backup" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filename: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::restore_db_backup(managed::<crate::store::AppState>(handle)?, args.filename).await)?))
        }
        "restore_env_backup" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            backup_path: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::restore_env_backup(args.backup_path))?))
        }
        "restore_skill_backup" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            backup_id: String,
            current_app: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::restore_skill_backup(args.backup_id, args.current_app, managed::<crate::store::AppState>(handle)?))?))
        }
        "run_tool_lifecycle_action" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            tools: Vec<String>,
            action: String,
            #[serde(default)]
            wsl_shell_by_tool: Option<std::collections::HashMap<String, crate::commands::misc::WslShellPreferenceInput>>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::run_tool_lifecycle_action(args.tools, args.action, args.wsl_shell_by_tool).await)?))
        }
        "s3_sync_download" => {
            Ok(Some(finish_result(crate::commands::s3_sync_download(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "s3_sync_fetch_remote_info" => {
            Ok(Some(finish_result(crate::commands::s3_sync_fetch_remote_info().await)?))
        }
        "s3_sync_save_settings" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            settings: crate::settings::S3SyncSettings,
            #[serde(default)]
            passwordTouched: Option<bool>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::s3_sync_save_settings(args.settings, args.passwordTouched).await)?))
        }
        "s3_sync_upload" => {
            Ok(Some(finish_result(crate::commands::s3_sync_upload(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "s3_test_connection" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            settings: crate::settings::S3SyncSettings,
            #[serde(default)]
            preserveEmptyPassword: Option<bool>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::s3_test_connection(args.settings, args.preserveEmptyPassword).await)?))
        }
        "save_models_dev_sync_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::services::model_pricing::ModelsDevSyncConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::save_models_dev_sync_config(managed::<crate::store::AppState>(handle)?, args.config))?))
        }
        "save_settings" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            settings: crate::settings::AppSettings,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::save_settings(managed::<crate::store::AppState>(handle)?, args.settings).await)?))
        }
        "scan_local_proxies" => {
            Ok(Some(finish(crate::commands::scan_local_proxies().await)?))
        }
        "scan_openclaw_config_health" => {
            Ok(Some(finish_result(crate::commands::scan_openclaw_config_health())?))
        }
        "scan_unmanaged_skills" => {
            Ok(Some(finish_result(crate::commands::scan_unmanaged_skills(managed::<crate::store::AppState>(handle)?))?))
        }
        "search_daily_memory_files" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            query: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::search_daily_memory_files(args.query).await)?))
        }
        "search_skills_sh" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            query: String,
            limit: usize,
            offset: usize,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::search_skills_sh(args.query, args.limit, args.offset).await)?))
        }
        "set_app_config_dir_override" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            #[serde(default)]
            path: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_app_config_dir_override(handle.clone(), args.path).await)?))
        }
        "set_auto_failover_enabled" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            enabled: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_auto_failover_enabled(handle.clone(), managed::<crate::store::AppState>(handle)?, args.app_type, args.enabled).await)?))
        }
        "set_auto_launch" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            enabled: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_auto_launch(args.enabled).await)?))
        }
        "set_claude_common_config_snippet" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            snippet: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_claude_common_config_snippet(args.snippet, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "set_common_config_snippet" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            snippet: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_common_config_snippet(args.app_type, args.snippet, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "set_copilot_optimizer_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::types::CopilotOptimizerConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_copilot_optimizer_config(managed::<crate::AppState>(handle)?, args.config).await)?))
        }
        "set_global_proxy_url" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            url: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_global_proxy_url(managed::<crate::store::AppState>(handle)?, args.url))?))
        }
        "set_hermes_memory" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            kind: crate::hermes_config::MemoryKind,
            content: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_hermes_memory(args.kind, args.content))?))
        }
        "set_hermes_memory_enabled" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            kind: crate::hermes_config::MemoryKind,
            enabled: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_hermes_memory_enabled(args.kind, args.enabled))?))
        }
        "set_log_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::types::LogConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_log_config(managed::<crate::AppState>(handle)?, args.config).await)?))
        }
        "set_mcp_enabled" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            enabled: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_mcp_enabled(managed::<crate::store::AppState>(handle)?, args.app, args.id, args.enabled).await)?))
        }
        "set_openclaw_agents_defaults" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            defaults: crate::openclaw_config::OpenClawAgentsDefaults,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_openclaw_agents_defaults(args.defaults))?))
        }
        "set_openclaw_default_model" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            model: crate::openclaw_config::OpenClawDefaultModel,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_openclaw_default_model(args.model))?))
        }
        "set_openclaw_env" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            env: crate::openclaw_config::OpenClawEnvConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_openclaw_env(args.env))?))
        }
        "set_openclaw_model_catalog" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            catalog: std::collections::HashMap<String, crate::openclaw_config::OpenClawModelCatalogEntry>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_openclaw_model_catalog(args.catalog))?))
        }
        "set_openclaw_tools" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            tools: crate::openclaw_config::OpenClawToolsConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_openclaw_tools(args.tools))?))
        }
        "set_optimizer_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::types::OptimizerConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_optimizer_config(managed::<crate::AppState>(handle)?, args.config).await)?))
        }
        "set_pricing_model_source" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            value: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_pricing_model_source(managed::<crate::store::AppState>(handle)?, args.app_type, args.value).await)?))
        }
        "set_proxy_takeover_for_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            enabled: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_proxy_takeover_for_app(managed::<crate::store::AppState>(handle)?, args.app_type, args.enabled).await)?))
        }
        "set_rectifier_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::types::RectifierConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::set_rectifier_config(managed::<crate::AppState>(handle)?, args.config).await)?))
        }
        "start_proxy_server" => {
            Ok(Some(finish_result(crate::commands::start_proxy_server(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "stop_proxy_server" => {
            Ok(Some(finish_result(crate::commands::stop_proxy_server(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "stop_proxy_with_restore" => {
            Ok(Some(finish_result(crate::commands::stop_proxy_with_restore(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "stream_check_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: crate::app_config::AppType,
            provider_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::stream_check_provider(managed::<crate::store::AppState>(handle)?, managed::<crate::commands::copilot::CopilotAuthState>(handle)?, args.app_type, args.provider_id).await)?))
        }
        "switch_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::switch_provider(handle.clone(), args.app, args.id).await)?))
        }
        "switch_proxy_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app_type: String,
            provider_id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::switch_proxy_provider(managed::<crate::store::AppState>(handle)?, args.app_type, args.provider_id).await)?))
        }
        "sync_current_providers_live" => {
            Ok(Some(finish_result(crate::commands::sync_current_providers_live(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "sync_session_usage" => {
            Ok(Some(finish_result(crate::commands::sync_session_usage(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "sync_universal_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::sync_universal_provider(handle.clone(), managed::<crate::store::AppState>(handle)?, args.id))?))
        }
        "testUsageScript" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            providerId: String,
            app: String,
            scriptCode: String,
            #[serde(default)]
            timeout: Option<u64>,
            #[serde(default)]
            apiKey: Option<String>,
            #[serde(default)]
            baseUrl: Option<String>,
            #[serde(default)]
            accessToken: Option<String>,
            #[serde(default)]
            userId: Option<String>,
            #[serde(default)]
            templateType: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::testUsageScript(managed::<crate::store::AppState>(handle)?, args.providerId, args.app, args.scriptCode, args.timeout, args.apiKey, args.baseUrl, args.accessToken, args.userId, args.templateType).await)?))
        }
        "test_api_endpoints" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            urls: Vec<String>,
            #[serde(default)]
            timeoutSecs: Option<u64>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::test_api_endpoints(args.urls, args.timeoutSecs).await)?))
        }
        "test_proxy_url" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            url: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::test_proxy_url(args.url).await)?))
        }
        "toggle_mcp_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            server_id: String,
            app: String,
            enabled: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::toggle_mcp_app(managed::<crate::store::AppState>(handle)?, args.server_id, args.app, args.enabled).await)?))
        }
        "toggle_skill_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            app: String,
            enabled: bool,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::toggle_skill_app(args.id, args.app, args.enabled, managed::<crate::store::AppState>(handle)?))?))
        }
        "uninstall_skill" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            directory: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::uninstall_skill(args.directory, managed::<crate::store::AppState>(handle)?))?))
        }
        "uninstall_skill_for_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            directory: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::uninstall_skill_for_app(args.app, args.directory, managed::<crate::store::AppState>(handle)?))?))
        }
        "uninstall_skill_unified" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::uninstall_skill_unified(args.id, managed::<crate::store::AppState>(handle)?))?))
        }
        "update_circuit_breaker_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::CircuitBreakerConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_circuit_breaker_config(managed::<crate::store::AppState>(handle)?, args.config).await)?))
        }
        "update_endpoint_last_used" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            providerId: String,
            url: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_endpoint_last_used(managed::<crate::store::AppState>(handle)?, args.app, args.providerId, args.url))?))
        }
        "update_global_proxy_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::types::GlobalProxyConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_global_proxy_config(managed::<crate::store::AppState>(handle)?, args.config).await)?))
        }
        "update_model_pricing" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            model_id: String,
            display_name: String,
            input_cost: String,
            output_cost: String,
            cache_read_cost: String,
            cache_creation_cost: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_model_pricing(managed::<crate::store::AppState>(handle)?, args.model_id, args.display_name, args.input_cost, args.output_cost, args.cache_read_cost, args.cache_creation_cost))?))
        }
        "update_model_pricing_batch" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            entries: Vec<crate::services::model_pricing::ModelPricingInfo>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_model_pricing_batch(managed::<crate::store::AppState>(handle)?, args.entries))?))
        }
        "update_pi_provider_usage_script" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            usageScript: crate::provider::UsageScript,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_pi_provider_usage_script(managed::<crate::store::AppState>(handle)?, args.id, args.usageScript))?))
        }
        "update_profile" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            #[serde(default)]
            name: Option<String>,
            #[serde(default)]
            resnapshot: Option<bool>,
            #[serde(default)]
            scope: Option<String>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_profile(managed::<crate::store::AppState>(handle)?, args.id, args.name, args.resnapshot, args.scope))?))
        }
        "update_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            provider: crate::provider::Provider,
            #[serde(default)]
            originalId: Option<String>,
            #[serde(default)]
            editorSave: Option<crate::services::provider::EditorSave>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_provider(handle.clone(), args.app, args.provider, args.originalId, args.editorSave).await)?))
        }
        "update_providers_sort_order" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            updates: Vec<crate::services::ProviderSortUpdate>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_providers_sort_order(managed::<crate::store::AppState>(handle)?, args.app, args.updates))?))
        }
        "update_proxy_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::types::ProxyConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_proxy_config(managed::<crate::store::AppState>(handle)?, args.config).await)?))
        }
        "update_proxy_config_for_app" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            config: crate::proxy::types::AppProxyConfig,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_proxy_config_for_app(managed::<crate::store::AppState>(handle)?, args.config).await)?))
        }
        "update_skill" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::update_skill(args.id, managed::<crate::commands::skill::SkillServiceState>(handle)?, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "upsert_claude_mcp_server" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            id: String,
            spec: serde_json::Value,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::upsert_claude_mcp_server(args.id, args.spec).await)?))
        }
        "upsert_mcp_server" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            server: crate::app_config::McpServer,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::upsert_mcp_server(managed::<crate::store::AppState>(handle)?, args.server).await)?))
        }
        "upsert_mcp_server_in_config" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            spec: serde_json::Value,
            #[serde(default)]
            sync_other_side: Option<bool>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::upsert_mcp_server_in_config(managed::<crate::store::AppState>(handle)?, args.app, args.id, args.spec, args.sync_other_side).await)?))
        }
        "upsert_pi_prompt_template" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            slug: String,
            #[serde(default)]
            originalSlug: Option<String>,
            expectedRevision: String,
            content: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::upsert_pi_prompt_template(args.slug, args.originalSlug, args.expectedRevision, args.content).await)?))
        }
        "upsert_prompt" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            app: String,
            id: String,
            prompt: crate::prompt::Prompt,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::upsert_prompt(args.app, args.id, args.prompt, managed::<crate::store::AppState>(handle)?).await)?))
        }
        "upsert_universal_provider" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            provider: crate::provider::UniversalProvider,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::upsert_universal_provider(handle.clone(), managed::<crate::store::AppState>(handle)?, args.provider))?))
        }
        "validate_mcp_command" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            cmd: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::validate_mcp_command(args.cmd).await)?))
        }
        "webdav_sync_download" => {
            Ok(Some(finish_result(crate::commands::webdav_sync_download(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "webdav_sync_fetch_remote_info" => {
            Ok(Some(finish_result(crate::commands::webdav_sync_fetch_remote_info().await)?))
        }
        "webdav_sync_save_settings" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            settings: crate::settings::WebDavSyncSettings,
            #[serde(default)]
            passwordTouched: Option<bool>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::webdav_sync_save_settings(args.settings, args.passwordTouched).await)?))
        }
        "webdav_sync_upload" => {
            Ok(Some(finish_result(crate::commands::webdav_sync_upload(managed::<crate::store::AppState>(handle)?).await)?))
        }
        "webdav_test_connection" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            settings: crate::settings::WebDavSyncSettings,
            #[serde(default)]
            preserveEmptyPassword: Option<bool>,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::webdav_test_connection(args.settings, args.preserveEmptyPassword).await)?))
        }
        "write_daily_memory_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filename: String,
            content: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::write_daily_memory_file(args.filename, args.content).await)?))
        }
        "write_workspace_file" => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
            filename: String,
            content: String,
            }
            let args: Args = match serde_json::from_value(payload) {
                Ok(value) => value,
                Err(error) => return Err(format!("参数无法解析: {error}")),
            };
            Ok(Some(finish_result(crate::commands::write_workspace_file(args.filename, args.content).await)?))
        }
        _ => Ok(None),
    }
}

fn managed<T: Send + Sync + 'static>(handle: &AppHandle) -> Result<State<'static, T>, String> {
    handle.try_state().ok_or_else(|| format!("缺少状态 {}", std::any::type_name::<T>()))
}

fn finish<T: Serialize>(value: T) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|error| error.to_string())
}

fn finish_result<T: Serialize, E: ToString>(value: Result<T, E>) -> Result<Value, String> {
    finish(value.map_err(|error| error.to_string())?)

}
