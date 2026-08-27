use codex_core::config::Config;
use codex_login::AuthManager;

/// Startup guard for memories extraction.
///
/// The original implementation queried Codex backend rate limits, which only
/// applies to ChatGPT account mode. This build targets API-key usage only, so
/// the guard always allows startup.
pub(crate) async fn rate_limits_ok(_auth_manager: &AuthManager, _config: &Config) -> bool {
    true
}
