use std::sync::Arc;

use crate::connection_manager::McpConnectionSet;
use crate::runtime::McpRuntimeInput;
use crate::server::McpServerMetadata;
use crate::server::McpServerOrigin;
use crate::tools::ToolInfo;
use codex_login::AuthManager;
use codex_login::CodexAuth;
use serde_json::Map;
use serde_json::Value;

pub(crate) const ENTITLEMENT_CONTEXT_KEY: &str = "openai/entitlementContext";
const REQUESTED_ENTITLEMENTS_KEY: &str = "openai/requestedEntitlements";
const CYBER_TRUSTED_ACCESS_ENTITLEMENT: &str = "cyber_trusted_access";

impl McpConnectionSet {
    /// Installed and task-selected plugins may request supported advisory entitlement metadata.
    /// Model calls use the local, read-only, zero-argument boundary.
    pub(crate) async fn add_trusted_access_context(
        &self,
        tool: &ToolInfo,
        server: &McpServerMetadata,
        arguments: Option<&Value>,
        meta: Option<Value>,
    ) -> Option<Value> {
        if tool
            .tool
            .meta
            .as_deref()
            .and_then(|meta| meta.get(REQUESTED_ENTITLEMENTS_KEY))
            .and_then(Value::as_array)
            .is_some_and(|entitlements| {
                entitlements.iter().all(Value::is_string)
                    && entitlements.iter().any(|entitlement| {
                        entitlement.as_str() == Some(CYBER_TRUSTED_ACCESS_ENTITLEMENT)
                    })
            })
            && self
                .plugin_id_for_mcp_server_name(&tool.server_name)
                .is_some()
            && arguments.is_none_or(|arguments| arguments.as_object().is_some_and(Map::is_empty))
            && server.environment_id == codex_config::DEFAULT_MCP_SERVER_ENVIRONMENT_ID
            && matches!(server.origin, Some(McpServerOrigin::Stdio))
            && tool
                .tool
                .annotations
                .as_ref()
                .and_then(|annotations| annotations.read_only_hint)
                == Some(true)
            && let Some(context) = self.trusted_access.as_ref()
        {
            context.add_context(meta).await
        } else {
            meta
        }
    }
}

/// Account-bound verified access context.
///
/// Verified access was tied to ChatGPT account auth; this API-key-only build
/// never constructs a context and never attaches entitlement metadata.
pub struct TrustedAccessContext {
    _auth: CodexAuth,
    _auth_manager: Arc<AuthManager>,
}

impl TrustedAccessContext {
    pub(crate) fn from_runtime(_input: &McpRuntimeInput) -> Option<Self> {
        None
    }

    /// Replaces caller-supplied entitlement metadata with a fresh verified result.
    pub async fn add_context(&self, meta: Option<Value>) -> Option<Value> {
        meta
    }
}
