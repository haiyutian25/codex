use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use codex_config::types::AuthCredentialsStoreMode;
use codex_login::AuthDotJson;
use codex_login::AuthKeyringBackendKind;
use codex_login::save_auth;
use codex_protocol::auth::AuthMode;

/// Builder for writing a fake auth.json in tests.
///
/// API-key-only build: the historical ChatGPT token fixture surface is
/// retained for call-site compatibility, but the written payload is always an
/// API-key auth record.
#[derive(Debug, Clone)]
pub struct ChatGptAuthFixture {
    access_token: String,
}

impl ChatGptAuthFixture {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
        }
    }

    pub fn refresh_token(self, _refresh_token: impl Into<String>) -> Self {
        self
    }

    pub fn account_id(self, _account_id: impl Into<String>) -> Self {
        self
    }

    pub fn plan_type(self, _plan_type: impl Into<String>) -> Self {
        self
    }

    pub fn chatgpt_user_id(self, _chatgpt_user_id: impl Into<String>) -> Self {
        self
    }

    pub fn chatgpt_account_id(self, _chatgpt_account_id: impl Into<String>) -> Self {
        self
    }

    pub fn email(self, _email: impl Into<String>) -> Self {
        self
    }

    pub fn last_refresh(self, _last_refresh: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self
    }
}

pub fn write_chatgpt_auth(
    codex_home: &Path,
    fixture: ChatGptAuthFixture,
    cli_auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> Result<()> {
    let auth = AuthDotJson {
        auth_mode: Some(AuthMode::ApiKey),
        openai_api_key: Some(fixture.access_token),
    };

    save_auth(
        codex_home,
        &auth,
        cli_auth_credentials_store_mode,
        AuthKeyringBackendKind::default(),
    )
    .context("write auth.json")
}
