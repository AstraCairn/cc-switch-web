//! OAuth manager handles shared by the proxy and the desktop command layer.
//! They live outside the Tauri command module so the web build can compile
//! the proxy without a window runtime.

use std::sync::Arc;

use tokio::sync::RwLock;

use super::providers::codex_oauth_auth::CodexOAuthManager;
use super::providers::copilot_auth::CopilotAuthManager;
use super::providers::xai_oauth_auth::XaiOAuthManager;

pub struct CodexOAuthState(pub Arc<CodexOAuthManager>);

pub struct CopilotAuthState(pub Arc<RwLock<CopilotAuthManager>>);

pub struct XaiOAuthState(pub Arc<RwLock<XaiOAuthManager>>);
