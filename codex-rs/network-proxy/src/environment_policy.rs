use crate::NetworkDomainPermissions;
use crate::NetworkProxyConfig;
use serde::Deserialize;
use serde::Serialize;

/// Traffic restrictions supplied by the owner of one execution environment.
///
/// Proxy enablement, listeners, network mode, MITM, and credentials remain outside
/// attachment-owned traffic policy.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentNetworkPolicy {
    pub domains: Option<NetworkDomainPermissions>,
    pub allow_upstream_proxy: bool,
    pub allow_local_binding: bool,
    pub managed_allowed_domains_only: bool,
}

impl EnvironmentNetworkPolicy {
    /// Captures portable traffic restrictions without exposing controller runtime settings.
    pub fn from_config(config: &NetworkProxyConfig, managed_allowed_domains_only: bool) -> Self {
        Self {
            domains: config.domains.clone(),
            allow_upstream_proxy: config.allow_upstream_proxy,
            allow_local_binding: config.allow_local_binding,
            managed_allowed_domains_only,
        }
    }

    /// Applies attachment-owned traffic settings while preserving inherited denials and proxy setup.
    pub fn apply_to(&self, config: &mut NetworkProxyConfig) {
        // Use the owner's domain rules without dropping controller denials.
        let inherited_denials = config.denied_domains().unwrap_or_default();
        config.domains.clone_from(&self.domains);
        for domain in inherited_denials {
            config.upsert_domain_permission(
                domain,
                crate::NetworkDomainPermission::Deny,
                crate::normalize_host,
            );
        }
        config.allow_upstream_proxy &= self.allow_upstream_proxy;
        config.allow_local_binding &= self.allow_local_binding;
    }
}
