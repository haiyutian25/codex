use std::env;
use std::fmt::Debug;
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use tokio::sync::watch;

use codex_protocol::auth::AuthMode;
use codex_protocol::config_types::ModelProviderAuthInfo;

use super::external_bearer::BearerTokenRefresher;
pub use crate::auth::storage::AuthDotJson;
pub use crate::auth::storage::AuthKeyringBackendKind;
use crate::auth::storage::create_auth_storage;
use crate::outbound_proxy::AuthRouteConfig;
use codex_config::types::AuthCredentialsStoreMode;
use codex_protocol::auth::RefreshTokenFailedError;
use codex_protocol::auth::RefreshTokenFailedReason;
use thiserror::Error;

/// Authentication mechanism used by the current user.
///
/// This build supports API-key authentication only.
#[derive(Debug, Clone)]
pub enum CodexAuth {
    ApiKey(ApiKeyAuth),
}

#[derive(Debug, Clone)]
pub struct ApiKeyAuth {
    api_key: String,
}

impl PartialEq for CodexAuth {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ApiKey(a), Self::ApiKey(b)) => a.api_key == b.api_key,
        }
    }
}

#[derive(Debug, Error)]
pub enum RefreshTokenError {
    #[error("{0}")]
    Permanent(#[from] RefreshTokenFailedError),
    #[error(transparent)]
    Transient(#[from] std::io::Error),
}

/// Error returned when constructing an [`AuthManager`] from resolved configuration.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct AuthManagerInitializationError(AuthManagerInitializationErrorSource);

#[derive(Debug, Error)]
enum AuthManagerInitializationErrorSource {
    #[error(transparent)]
    InitialAuth(RefreshTokenError),
}

impl From<RefreshTokenError> for AuthManagerInitializationError {
    fn from(error: RefreshTokenError) -> Self {
        Self(AuthManagerInitializationErrorSource::InitialAuth(error))
    }
}

impl From<AuthManagerInitializationError> for std::io::Error {
    fn from(error: AuthManagerInitializationError) -> Self {
        Self::other(error)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternalAuthRefreshReason {
    Unauthorized,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalAuthRefreshContext {
    pub reason: ExternalAuthRefreshReason,
    pub previous_account_id: Option<String>,
}

/// Pluggable auth provider used by `AuthManager` for externally managed auth flows.
///
/// Implementations own the current auth value and any source-specific refresh mechanism.
pub trait ExternalAuth: Send + Sync {
    /// Returns the provider's current auth value.
    fn resolve(&self) -> ExternalAuthFuture<'_, CodexAuth>;

    /// Refreshes auth and makes the returned value current for future `resolve()` calls.
    fn refresh(&self, context: ExternalAuthRefreshContext) -> ExternalAuthFuture<'_, CodexAuth>;

    /// Maps a provider error into the retry policy used by external-auth reload and recovery.
    fn classify_error(&self, error: std::io::Error) -> RefreshTokenError {
        RefreshTokenError::Transient(error)
    }
}

pub type ExternalAuthFuture<'a, T> = Pin<Box<dyn Future<Output = std::io::Result<T>> + Send + 'a>>;

impl RefreshTokenError {
    pub fn failed_reason(&self) -> Option<RefreshTokenFailedReason> {
        match self {
            Self::Permanent(error) => Some(error.reason),
            Self::Transient(_) => None,
        }
    }
}

impl From<RefreshTokenError> for std::io::Error {
    fn from(err: RefreshTokenError) -> std::io::Error {
        match err {
            RefreshTokenError::Permanent(failed) => std::io::Error::other(failed),
            RefreshTokenError::Transient(inner) => inner,
        }
    }
}

/// Result of a single unauthorized-recovery step.
pub struct UnauthorizedRecoveryStepResult {
    auth_state_changed: Option<bool>,
}

impl UnauthorizedRecoveryStepResult {
    pub fn auth_state_changed(&self) -> Option<bool> {
        self.auth_state_changed
    }
}

/// Unauthorized-recovery ladder for 401 responses.
///
/// API-key-only build: API keys do not expire, so no recovery steps exist.
/// The type is retained because the API client's 401 handling is generic.
pub struct UnauthorizedRecovery;

impl UnauthorizedRecovery {
    pub fn has_next(&self) -> bool {
        false
    }

    pub fn mode_name(&self) -> &'static str {
        "none"
    }

    pub fn step_name(&self) -> &'static str {
        "none"
    }

    pub fn unavailable_reason(&self) -> &'static str {
        "api_key_auth_has_no_recovery"
    }

    pub async fn next(&mut self) -> Result<UnauthorizedRecoveryStepResult, RefreshTokenError> {
        Err(RefreshTokenError::Transient(std::io::Error::other(
            "no recovery steps available",
        )))
    }
}

impl CodexAuth {
    fn from_auth_dot_json(auth_dot_json: AuthDotJson) -> std::io::Result<Option<Self>> {
        let AuthDotJson {
            auth_mode,
            openai_api_key,
        } = auth_dot_json;
        match auth_mode {
            Some(AuthMode::ApiKey) | None => {
                let Some(api_key) = openai_api_key.filter(|key| !key.trim().is_empty()) else {
                    return Ok(None);
                };
                Ok(Some(Self::ApiKey(ApiKeyAuth { api_key })))
            }
            // Legacy account-mode payloads are ignored in this build.
            Some(_) => Ok(None),
        }
    }

    /// Load auth from the configured storage backend for `codex_home`.
    pub async fn from_auth_storage(
        codex_home: PathBuf,
        auth_credentials_store_mode: AuthCredentialsStoreMode,
        keyring_backend_kind: AuthKeyringBackendKind,
    ) -> std::io::Result<Option<Self>> {
        let storage = create_auth_storage(codex_home, auth_credentials_store_mode, keyring_backend_kind);
        let Some(auth_dot_json) = storage.load()? else {
            return Ok(None);
        };
        Self::from_auth_dot_json(auth_dot_json)
    }

    pub fn auth_mode(&self) -> AuthMode {
        match self {
            Self::ApiKey(_) => AuthMode::ApiKey,
        }
    }

    /// Returns the precise kind of credentials backing this authentication.
    pub fn api_auth_mode(&self) -> AuthMode {
        self.auth_mode()
    }

    pub fn is_api_key_auth(&self) -> bool {
        true
    }

    pub fn uses_codex_backend(&self) -> bool {
        false
    }

    /// Returns the API key backing this authentication.
    pub fn api_key(&self) -> Option<&str> {
        match self {
            Self::ApiKey(auth) => Some(auth.api_key.as_str()),
        }
    }

    /// Returns the token string used for bearer authentication.
    pub fn get_token(&self) -> Result<String, std::io::Error> {
        match self {
            Self::ApiKey(auth) => Ok(auth.api_key.clone()),
        }
    }

    /// Adds bearer authorization headers for this auth to `headers`.
    pub fn add_auth_headers(
        &self,
        headers: &mut http::HeaderMap,
    ) -> Result<(), http::Error> {
        // API-key auth always has a token; a missing token degrades to an
        // empty bearer value rather than a construction failure.
        let token = self.get_token().unwrap_or_default();
        let mut value = http::HeaderValue::try_from(format!("Bearer {token}"))?;
        value.set_sensitive(true);
        headers.insert(http::header::AUTHORIZATION, value);
        Ok(())
    }

    /// Builds a fresh header map containing bearer authorization for this auth.
    pub fn to_auth_headers(&self) -> Result<http::HeaderMap, http::Error> {
        let mut headers = http::HeaderMap::new();
        self.add_auth_headers(&mut headers)?;
        Ok(headers)
    }

    pub fn from_api_key(api_key: &str) -> Self {
        Self::ApiKey(ApiKeyAuth {
            api_key: api_key.to_string(),
        })
    }
}

pub const OPENAI_API_KEY_ENV_VAR: &str = "OPENAI_API_KEY";
pub const CODEX_API_KEY_ENV_VAR: &str = "CODEX_API_KEY";
pub const CODEX_ACCESS_TOKEN_ENV_VAR: &str = "CODEX_ACCESS_TOKEN";

pub fn read_openai_api_key_from_env() -> Option<String> {
    env::var(OPENAI_API_KEY_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn read_codex_api_key_from_env() -> Option<String> {
    read_non_empty_env_var(CODEX_API_KEY_ENV_VAR)
}

pub fn read_codex_access_token_from_env() -> Option<String> {
    read_non_empty_env_var(CODEX_ACCESS_TOKEN_ENV_VAR)
}

fn read_non_empty_env_var(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Delete stored credentials inside `codex_home` if they exist. Returns
/// `Ok(true)` if something was removed, `Ok(false)` if nothing was stored.
pub fn logout(
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    keyring_backend_kind: AuthKeyringBackendKind,
) -> std::io::Result<bool> {
    let storage = create_auth_storage(
        codex_home.to_path_buf(),
        auth_credentials_store_mode,
        keyring_backend_kind,
    );
    storage.delete()
}

/// Writes an `auth.json` that contains only the API key.
pub fn login_with_api_key(
    codex_home: &Path,
    api_key: &str,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    keyring_backend_kind: AuthKeyringBackendKind,
) -> std::io::Result<()> {
    let auth_dot_json = AuthDotJson {
        auth_mode: Some(AuthMode::ApiKey),
        openai_api_key: Some(api_key.to_string()),
    };
    save_auth(
        codex_home,
        &auth_dot_json,
        auth_credentials_store_mode,
        keyring_backend_kind,
    )
}

/// Persist the provided auth payload using the specified backend.
pub fn save_auth(
    codex_home: &Path,
    auth: &AuthDotJson,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    keyring_backend_kind: AuthKeyringBackendKind,
) -> std::io::Result<()> {
    let storage = create_auth_storage(
        codex_home.to_path_buf(),
        auth_credentials_store_mode,
        keyring_backend_kind,
    );
    storage.save(auth)
}

/// Load the raw stored auth payload without applying environment overrides.
///
/// Returns `None` when no credentials are stored. Prefer `AuthManager` for
/// ordinary production reads; this helper is for tests and write-side
/// maintenance that must inspect the exact payload in storage.
pub fn load_auth_dot_json(
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    keyring_backend_kind: AuthKeyringBackendKind,
) -> std::io::Result<Option<AuthDotJson>> {
    let storage = create_auth_storage(
        codex_home.to_path_buf(),
        auth_credentials_store_mode,
        keyring_backend_kind,
    );
    storage.load()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthConfig {
    pub codex_home: PathBuf,
    pub auth_credentials_store_mode: AuthCredentialsStoreMode,
    pub keyring_backend_kind: AuthKeyringBackendKind,
    pub auth_route_config: AuthRouteConfig,
}

impl AuthConfig {
    pub fn validate(&self) -> std::io::Result<()> {
        Ok(())
    }

    pub async fn load_auth(
        &self,
        enable_codex_api_key_env: bool,
    ) -> std::io::Result<Option<CodexAuth>> {
        if enable_codex_api_key_env {
            if let Some(api_key) = read_codex_api_key_from_env() {
                return Ok(Some(CodexAuth::from_api_key(&api_key)));
            }
        }
        if let Some(api_key) = read_openai_api_key_from_env() {
            return Ok(Some(CodexAuth::from_api_key(&api_key)));
        }
        CodexAuth::from_auth_storage(
            self.codex_home.clone(),
            self.auth_credentials_store_mode,
            self.keyring_backend_kind,
        )
        .await
    }
}

/// No-op retained for API compatibility: API-key-only builds have no
/// login-method restrictions to enforce.
pub async fn enforce_login_restrictions(_config: &AuthConfig) -> std::io::Result<()> {
    Ok(())
}

#[derive(Debug, Default)]
struct CachedAuth {
    auth: Option<CodexAuth>,
}

static NEXT_AUTH_CHANGE: AtomicU64 = AtomicU64::new(1);

pub struct AuthManager {
    codex_home: PathBuf,
    inner: RwLock<CachedAuth>,
    auth_change_tx: watch::Sender<u64>,
    enable_codex_api_key_env: bool,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    keyring_backend_kind: AuthKeyringBackendKind,
    external_auth: RwLock<Option<Arc<dyn ExternalAuth>>>,
    auth_route_config: AuthRouteConfig,
}

/// Configuration view required to construct a shared [`AuthManager`].
///
/// Implementations should return the auth-related config values for the
/// already-resolved runtime configuration. The primary implementation is
/// `codex_core::config::Config`, but this trait keeps `codex-login` independent
/// from `codex-core`.
pub trait AuthManagerConfig {
    /// Returns the Codex home directory used for auth storage.
    fn codex_home(&self) -> PathBuf;

    /// Returns the CLI auth credential storage mode for auth loading.
    fn cli_auth_credentials_store_mode(&self) -> AuthCredentialsStoreMode;

    /// Returns the backend to use when CLI auth keyring storage is selected.
    fn auth_keyring_backend_kind(&self) -> AuthKeyringBackendKind;

    /// Returns route-selection settings for auth-owned clients.
    fn auth_route_config(&self) -> AuthRouteConfig;
}

impl Debug for AuthManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthManager")
            .field("codex_home", &self.codex_home)
            .field("inner", &self.inner)
            .field("enable_codex_api_key_env", &self.enable_codex_api_key_env)
            .field(
                "auth_credentials_store_mode",
                &self.auth_credentials_store_mode,
            )
            .field("keyring_backend_kind", &self.keyring_backend_kind)
            .field("has_external_auth", &self.has_external_auth())
            .finish_non_exhaustive()
    }
}

impl AuthManager {
    /// Create a new manager loading the initial auth. Errors loading auth are
    /// swallowed; `auth()` will simply return `None` in that case so callers
    /// can treat it as an unauthenticated state.
    pub async fn new(
        codex_home: PathBuf,
        enable_codex_api_key_env: bool,
        auth_credentials_store_mode: AuthCredentialsStoreMode,
        keyring_backend_kind: AuthKeyringBackendKind,
        auth_route_config: AuthRouteConfig,
    ) -> Self {
        Self::new_from_auth_config(
            AuthConfig {
                codex_home,
                auth_credentials_store_mode,
                keyring_backend_kind,
                auth_route_config,
            },
            enable_codex_api_key_env,
        )
        .await
    }

    async fn new_from_auth_config(auth_config: AuthConfig, enable_codex_api_key_env: bool) -> Self {
        let managed_auth = auth_config
            .load_auth(enable_codex_api_key_env)
            .await
            .ok()
            .flatten();
        let AuthConfig {
            codex_home,
            auth_credentials_store_mode,
            keyring_backend_kind,
            auth_route_config,
        } = auth_config;
        let (auth_change_tx, _auth_change_rx) = watch::channel(0);
        Self {
            codex_home,
            inner: RwLock::new(CachedAuth { auth: managed_auth }),
            auth_change_tx,
            enable_codex_api_key_env,
            auth_credentials_store_mode,
            keyring_backend_kind,
            external_auth: RwLock::new(None),
            auth_route_config,
        }
    }

    /// Create an AuthManager with a specific CodexAuth, for testing only.
    pub fn from_auth_for_testing(auth: CodexAuth) -> Arc<Self> {
        Self::from_optional_auth_for_testing(Some(auth))
    }

    /// Create an AuthManager with an optional CodexAuth, for testing only.
    pub(crate) fn from_optional_auth_for_testing(auth: Option<CodexAuth>) -> Arc<Self> {
        let cached = CachedAuth { auth };
        let (auth_change_tx, _auth_change_rx) = watch::channel(0);

        Arc::new(Self {
            codex_home: PathBuf::from("non-existent"),
            inner: RwLock::new(cached),
            auth_change_tx,
            enable_codex_api_key_env: false,
            auth_credentials_store_mode: AuthCredentialsStoreMode::File,
            keyring_backend_kind: AuthKeyringBackendKind::default(),
            external_auth: RwLock::new(None),
            auth_route_config: AuthRouteConfig::default(),
        })
    }

    /// Create an AuthManager with a specific CodexAuth and codex_home, for testing only.
    pub fn from_auth_for_testing_with_home(auth: CodexAuth, codex_home: PathBuf) -> Arc<Self> {
        let cached = CachedAuth { auth: Some(auth) };
        let (auth_change_tx, _auth_change_rx) = watch::channel(0);

        Arc::new(Self {
            codex_home,
            inner: RwLock::new(cached),
            auth_change_tx,
            enable_codex_api_key_env: false,
            auth_credentials_store_mode: AuthCredentialsStoreMode::File,
            keyring_backend_kind: AuthKeyringBackendKind::default(),
            external_auth: RwLock::new(None),
            auth_route_config: AuthRouteConfig::default(),
        })
    }

    /// Create an AuthManager that resolves auth exclusively from an external
    /// bearer-token provider command.
    pub fn external_bearer_only(config: ModelProviderAuthInfo) -> Arc<Self> {
        let (auth_change_tx, _auth_change_rx) = watch::channel(0);
        let external_auth: Arc<dyn ExternalAuth> = Arc::new(BearerTokenRefresher::new(config));
        Arc::new(Self {
            codex_home: PathBuf::from("non-existent"),
            inner: RwLock::new(CachedAuth::default()),
            auth_change_tx,
            enable_codex_api_key_env: false,
            auth_credentials_store_mode: AuthCredentialsStoreMode::File,
            keyring_backend_kind: AuthKeyringBackendKind::default(),
            external_auth: RwLock::new(Some(external_auth)),
            auth_route_config: AuthRouteConfig::default(),
        })
    }

    pub fn auth_cached(&self) -> Option<CodexAuth> {
        self.inner.read().map(|guard| guard.auth.clone()).ok()?
    }

    pub fn auth_change_receiver(&self) -> watch::Receiver<u64> {
        self.auth_change_tx.subscribe()
    }

    pub fn refresh_failure_for_auth(&self, _auth: &CodexAuth) -> Option<RefreshTokenFailedError> {
        None
    }

    pub async fn auth(&self) -> Option<CodexAuth> {
        if let Some(auth) = self.auth_cached() {
            return Some(auth);
        }
        self.resolve_external_auth().await
    }

    pub async fn reload(&self) -> bool {
        let external = self
            .external_auth
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_default();
        let Some(external) = external else {
            let refreshed = AuthConfig {
                codex_home: self.codex_home.clone(),
                auth_credentials_store_mode: self.auth_credentials_store_mode,
                keyring_backend_kind: self.keyring_backend_kind,
                auth_route_config: self.auth_route_config.clone(),
            }
            .load_auth(self.enable_codex_api_key_env)
            .await
            .ok()
            .flatten();
            return self.apply_auth(refreshed);
        };
        match external.resolve().await {
            Ok(auth) => self.apply_auth(Some(auth)),
            Err(_) => false,
        }
    }

    fn apply_auth(&self, auth: Option<CodexAuth>) -> bool {
        let Ok(mut guard) = self.inner.write() else {
            return false;
        };
        let changed = guard.auth != auth;
        guard.auth = auth;
        if changed {
            let _ = self
                .auth_change_tx
                .send(NEXT_AUTH_CHANGE.fetch_add(1, Ordering::SeqCst));
        }
        changed
    }

    pub async fn set_external_auth(&self, external_auth: Arc<dyn ExternalAuth>) {
        {
            let mut guard = self.external_auth.write().unwrap();
            *guard = Some(external_auth);
        }
        let _ = self.reload().await;
    }

    pub fn clear_external_auth(&self) {
        let mut guard = self.external_auth.write().unwrap();
        *guard = None;
        let _ = self.apply_auth(None);
    }

    pub fn has_external_auth(&self) -> bool {
        self.external_auth
            .read()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    pub fn is_external_chatgpt_auth_active(&self) -> bool {
        false
    }

    pub fn codex_api_key_env_enabled(&self) -> bool {
        self.enable_codex_api_key_env
    }

    pub async fn shared(
        codex_home: PathBuf,
        enable_codex_api_key_env: bool,
        auth_credentials_store_mode: AuthCredentialsStoreMode,
        keyring_backend_kind: AuthKeyringBackendKind,
        auth_route_config: AuthRouteConfig,
    ) -> Arc<Self> {
        Arc::new(
            Self::new(
                codex_home,
                enable_codex_api_key_env,
                auth_credentials_store_mode,
                keyring_backend_kind,
                auth_route_config,
            )
            .await,
        )
    }

    pub async fn shared_from_config(
        config: &impl AuthManagerConfig,
        enable_codex_api_key_env: bool,
    ) -> std::io::Result<Arc<Self>> {
        Self::shared_from_auth_config(
            AuthConfig {
                codex_home: config.codex_home(),
                auth_credentials_store_mode: config.cli_auth_credentials_store_mode(),
                keyring_backend_kind: config.auth_keyring_backend_kind(),
                auth_route_config: config.auth_route_config(),
            },
            enable_codex_api_key_env,
        )
        .await
    }

    pub async fn shared_from_auth_config(
        auth_config: AuthConfig,
        enable_codex_api_key_env: bool,
    ) -> std::io::Result<Arc<Self>> {
        auth_config.validate()?;
        Ok(Arc::new(
            Self::new_from_auth_config(auth_config, enable_codex_api_key_env).await,
        ))
    }

    async fn resolve_external_auth(&self) -> Option<CodexAuth> {
        let external = self
            .external_auth
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_default()?;
        external.resolve().await.ok()
    }

    /// API keys never expire; retained for interface compatibility.
    pub async fn refresh_token(&self) -> Result<(), RefreshTokenError> {
        Ok(())
    }

    /// API keys never expire; retained for interface compatibility.
    pub async fn refresh_token_from_authority(&self) -> Result<(), RefreshTokenError> {
        Ok(())
    }

    /// Returns the unauthorized-recovery ladder (empty in this build).
    pub fn unauthorized_recovery(&self) -> UnauthorizedRecovery {
        UnauthorizedRecovery
    }

    pub async fn logout(&self) -> std::io::Result<bool> {
        logout(
            &self.codex_home,
            self.auth_credentials_store_mode,
            self.keyring_backend_kind,
        )
    }

    pub async fn logout_with_revoke(&self) -> std::io::Result<bool> {
        self.logout().await
    }

    pub fn get_api_auth_mode(&self) -> Option<AuthMode> {
        self.auth_cached().map(|auth| auth.api_auth_mode())
    }

    pub fn auth_mode(&self) -> Option<AuthMode> {
        self.auth_cached().map(|auth| auth.auth_mode())
    }

    pub fn current_auth_uses_codex_backend(&self) -> bool {
        false
    }
}
