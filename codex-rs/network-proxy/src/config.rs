use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::collections::BTreeMap;
use std::net::IpAddr;
use std::net::SocketAddr;
use tracing::warn;
use url::Url;

use crate::mitm_hook::MitmHookConfig;
use crate::policy::normalize_host;

/// Variant order encodes effective precedence for duplicate patterns:
/// `None < Allow < Deny`, so deny wins over allow when entries conflict.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum NetworkDomainPermission {
    None,
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkDomainPermissionEntry {
    pub pattern: String,
    pub permission: NetworkDomainPermission,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetworkDomainPermissions {
    pub entries: Vec<NetworkDomainPermissionEntry>,
}

impl Serialize for NetworkDomainPermissions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.effective_entries()
            .into_iter()
            .map(|entry| (entry.pattern, entry.permission))
            .collect::<BTreeMap<_, _>>()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NetworkDomainPermissions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries = BTreeMap::<String, NetworkDomainPermission>::deserialize(deserializer)?
            .into_iter()
            .map(|(pattern, permission)| NetworkDomainPermissionEntry {
                pattern,
                permission,
            })
            .collect();
        Ok(Self { entries })
    }
}

impl NetworkDomainPermissions {
    fn effective_entries(&self) -> Vec<NetworkDomainPermissionEntry> {
        let mut order = Vec::new();
        let mut effective_permissions = BTreeMap::new();

        for entry in &self.entries {
            if !effective_permissions.contains_key(&entry.pattern) {
                order.push(entry.pattern.clone());
            }

            let permission = effective_permissions
                .entry(entry.pattern.clone())
                .or_insert(entry.permission);
            if entry.permission > *permission {
                *permission = entry.permission;
            }
        }

        order
            .into_iter()
            .filter_map(|pattern| {
                effective_permissions.remove(&pattern).map(|permission| {
                    NetworkDomainPermissionEntry {
                        pattern,
                        permission,
                    }
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NetworkProxyConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_proxy_url")]
    pub proxy_url: String,
    pub enable_socks5: bool,
    #[serde(default = "default_socks_url")]
    pub socks_url: String,
    pub enable_socks5_udp: bool,
    pub allow_upstream_proxy: bool,
    #[serde(default)]
    pub dangerously_allow_non_loopback_proxy: bool,
    #[serde(default)]
    pub mode: NetworkMode,
    #[serde(default)]
    pub domains: Option<NetworkDomainPermissions>,
    pub allow_local_binding: bool,
    #[serde(default)]
    pub mitm: bool,
    #[serde(default)]
    pub credential_broker: bool,
    /// Trusted OpenAI endpoint derived from local configuration, never sent to remote executors.
    #[serde(skip)]
    pub credential_broker_openai_host: Option<String>,
    #[serde(default)]
    pub dangerously_allow_plaintext_credential_injection: bool,
    #[serde(default)]
    pub mitm_hooks: Vec<MitmHookConfig>,
}

impl Default for NetworkProxyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            proxy_url: default_proxy_url(),
            enable_socks5: true,
            socks_url: default_socks_url(),
            enable_socks5_udp: true,
            allow_upstream_proxy: true,
            dangerously_allow_non_loopback_proxy: false,
            mode: NetworkMode::default(),
            domains: None,
            allow_local_binding: false,
            mitm: false,
            credential_broker: false,
            credential_broker_openai_host: None,
            dangerously_allow_plaintext_credential_injection: false,
            mitm_hooks: Vec::new(),
        }
    }
}

impl NetworkProxyConfig {
    pub fn set_credential_broker_enabled(&mut self, enabled: bool) {
        self.credential_broker = enabled;
        self.mitm |= enabled;
    }

    pub fn set_credential_broker_openai_base_url(&mut self, base_url: Option<&str>) {
        self.credential_broker_openai_host = base_url.and_then(trusted_credential_broker_host);
    }

    pub fn allowed_domains(&self) -> Option<Vec<String>> {
        self.domain_entries(NetworkDomainPermission::Allow)
    }

    pub fn denied_domains(&self) -> Option<Vec<String>> {
        self.domain_entries(NetworkDomainPermission::Deny)
    }

    fn domain_entries(&self, permission: NetworkDomainPermission) -> Option<Vec<String>> {
        self.domains
            .as_ref()
            .map(|domains| {
                domains
                    .effective_entries()
                    .iter()
                    .filter(|entry| entry.permission == permission)
                    .map(|entry| entry.pattern.clone())
                    .collect()
            })
            .filter(|entries: &Vec<String>| !entries.is_empty())
    }

    pub fn set_allowed_domains(&mut self, allowed_domains: Vec<String>) {
        self.set_domain_entries(allowed_domains, NetworkDomainPermission::Allow);
    }

    pub fn set_denied_domains(&mut self, denied_domains: Vec<String>) {
        self.set_domain_entries(denied_domains, NetworkDomainPermission::Deny);
    }

    pub fn upsert_domain_permission(
        &mut self,
        host: String,
        permission: NetworkDomainPermission,
        normalize: impl Fn(&str) -> String,
    ) {
        let mut domains = self.domains.take().unwrap_or_default();
        let normalized_host = normalize(&host);
        domains
            .entries
            .retain(|entry| normalize(&entry.pattern) != normalized_host);
        domains.entries.push(NetworkDomainPermissionEntry {
            pattern: host,
            permission,
        });
        self.domains = (!domains.entries.is_empty()).then_some(domains);
    }

    fn set_domain_entries(&mut self, entries: Vec<String>, permission: NetworkDomainPermission) {
        let mut domains = self.domains.take().unwrap_or_default();
        domains
            .entries
            .retain(|entry| entry.permission != permission);
        for entry in entries {
            if !domains
                .entries
                .iter()
                .any(|existing| existing.pattern == entry && existing.permission == permission)
            {
                domains.entries.push(NetworkDomainPermissionEntry {
                    pattern: entry,
                    permission,
                });
            }
        }
        self.domains = (!domains.entries.is_empty()).then_some(domains);
    }
}

pub(crate) fn trusted_credential_broker_host(base_url: &str) -> Option<String> {
    Url::parse(base_url)
        .ok()
        .filter(|url| {
            url.scheme() == "https" && url.username().is_empty() && url.password().is_none()
        })
        .and_then(|url| url.host_str().map(normalize_host))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum NetworkMode {
    /// Limited (read-only) access: only GET/HEAD/OPTIONS are allowed for HTTP. HTTPS CONNECT is
    /// blocked unless MITM is enabled so the proxy can enforce method policy on inner requests.
    /// SOCKS5 UDP and non-HTTPS SOCKS5 TCP remain blocked in limited mode.
    Limited,
    /// Full network access: all HTTP methods are allowed. HTTPS CONNECTs are tunneled directly.
    /// MITM hooks do not currently make full mode enter MITM.
    #[default]
    Full,
}

impl NetworkMode {
    pub fn allows_method(self, method: &str) -> bool {
        match self {
            Self::Full => true,
            Self::Limited => matches!(method, "GET" | "HEAD" | "OPTIONS"),
        }
    }
}

fn default_proxy_url() -> String {
    "http://127.0.0.1:3128".to_string()
}

fn default_socks_url() -> String {
    "http://127.0.0.1:8081".to_string()
}

/// Clamp non-loopback bind addresses to loopback unless explicitly allowed.
fn clamp_non_loopback(
    addr: SocketAddr,
    allow_non_loopback: bool,
    name: &str,
    override_setting_name: &str,
) -> SocketAddr {
    if addr.ip().is_loopback() {
        return addr;
    }

    if allow_non_loopback {
        warn!("DANGEROUS: {name} listening on non-loopback address {addr}");
        return addr;
    }

    warn!(
        "{name} requested non-loopback bind ({addr}); clamping to 127.0.0.1:{port} (set {override_setting_name} to override)",
        port = addr.port()
    );
    SocketAddr::from(([127, 0, 0, 1], addr.port()))
}

pub(crate) fn clamp_bind_addrs(
    http_addr: SocketAddr,
    socks_addr: SocketAddr,
    cfg: &NetworkProxyConfig,
) -> (SocketAddr, SocketAddr) {
    let http_addr = clamp_non_loopback(
        http_addr,
        cfg.dangerously_allow_non_loopback_proxy,
        "HTTP proxy",
        "dangerously_allow_non_loopback_proxy",
    );
    let socks_addr = clamp_non_loopback(
        socks_addr,
        cfg.dangerously_allow_non_loopback_proxy,
        "SOCKS5 proxy",
        "dangerously_allow_non_loopback_proxy",
    );
    (http_addr, socks_addr)
}

pub struct RuntimeConfig {
    pub http_addr: SocketAddr,
    pub socks_addr: SocketAddr,
}

pub fn resolve_runtime(cfg: &NetworkProxyConfig) -> Result<RuntimeConfig> {
    let http_addr = resolve_addr(&cfg.proxy_url, /*default_port*/ 3128)
        .with_context(|| format!("invalid network.proxy_url: {}", cfg.proxy_url))?;
    let socks_addr = resolve_addr(&cfg.socks_url, /*default_port*/ 8081)
        .with_context(|| format!("invalid network.socks_url: {}", cfg.socks_url))?;
    let (http_addr, socks_addr) = clamp_bind_addrs(http_addr, socks_addr, cfg);

    Ok(RuntimeConfig {
        http_addr,
        socks_addr,
    })
}

/// Returns the sorted loopback ports used by the configured managed proxy listeners.
pub fn managed_proxy_ports(cfg: &NetworkProxyConfig) -> Result<Vec<u16>> {
    let runtime = resolve_runtime(cfg)?;
    if runtime.http_addr.port() == 0 {
        bail!("network.proxy_url must use a fixed non-zero port for managed proxy provisioning");
    }
    let mut ports = vec![runtime.http_addr.port()];
    if cfg.enable_socks5 {
        if runtime.socks_addr.port() == 0 {
            bail!(
                "network.socks_url must use a fixed non-zero port for managed proxy provisioning"
            );
        }
        ports.push(runtime.socks_addr.port());
    }
    ports.sort_unstable();
    ports.dedup();
    Ok(ports)
}

fn resolve_addr(url: &str, default_port: u16) -> Result<SocketAddr> {
    let addr_parts = parse_host_port(url, default_port)?;
    let host = if addr_parts.host.eq_ignore_ascii_case("localhost") {
        "127.0.0.1".to_string()
    } else {
        addr_parts.host
    };
    match host.parse::<IpAddr>() {
        Ok(ip) => Ok(SocketAddr::new(ip, addr_parts.port)),
        Err(_) => Ok(SocketAddr::from(([127, 0, 0, 1], addr_parts.port))),
    }
}

pub fn host_and_port_from_network_addr(value: &str, default_port: u16) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "<missing>".to_string();
    }

    let parts = match parse_host_port(trimmed, default_port) {
        Ok(parts) => parts,
        Err(_) => {
            return format_host_and_port(trimmed, default_port);
        }
    };

    format_host_and_port(&parts.host, parts.port)
}

fn format_host_and_port(host: &str, port: u16) -> String {
    if host.contains(':') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SocketAddressParts {
    host: String,
    port: u16,
}

fn parse_host_port(url: &str, default_port: u16) -> Result<SocketAddressParts> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        bail!("missing host in network proxy address: {url}");
    }

    // Avoid treating unbracketed IPv6 literals like "2001:db8::1" as scheme-prefixed URLs.
    if matches!(trimmed.parse::<IpAddr>(), Ok(IpAddr::V6(_))) && !trimmed.starts_with('[') {
        return Ok(SocketAddressParts {
            host: trimmed.to_string(),
            port: default_port,
        });
    }

    // Prefer the standard URL parser when the input is URL-like. Prefix a scheme when absent so
    // we still accept loose host:port inputs.
    let candidate = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("http://{trimmed}")
    };
    if let Ok(parsed) = Url::parse(&candidate)
        && let Some(host) = parsed.host_str()
    {
        let host = host.trim_matches(|c| c == '[' || c == ']');
        if host.is_empty() {
            bail!("missing host in network proxy address: {url}");
        }
        return Ok(SocketAddressParts {
            host: host.to_string(),
            port: parsed.port().unwrap_or(default_port),
        });
    }

    parse_host_port_fallback(trimmed, default_port)
}

fn parse_host_port_fallback(input: &str, default_port: u16) -> Result<SocketAddressParts> {
    let without_scheme = input
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(input);
    let host_port = without_scheme.split('/').next().unwrap_or(without_scheme);
    let host_port = host_port
        .rsplit_once('@')
        .map(|(_, rest)| rest)
        .unwrap_or(host_port);

    if host_port.starts_with('[')
        && let Some(end) = host_port.find(']')
    {
        let host = &host_port[1..end];
        let port = host_port[end + 1..]
            .strip_prefix(':')
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(default_port);
        if host.is_empty() {
            bail!("missing host in network proxy address: {input}");
        }
        return Ok(SocketAddressParts {
            host: host.to_string(),
            port,
        });
    }

    // Only treat `host:port` as such when there's a single `:`. This avoids
    // accidentally interpreting unbracketed IPv6 addresses as `host:port`.
    if host_port.bytes().filter(|b| *b == b':').count() == 1
        && let Some((host, port)) = host_port.rsplit_once(':')
    {
        if host.is_empty() {
            bail!("missing host in network proxy address: {input}");
        }
        return Ok(SocketAddressParts {
            host: host.to_string(),
            port: port.parse::<u16>().ok().unwrap_or(default_port),
        });
    }

    if host_port.is_empty() {
        bail!("missing host in network proxy address: {input}");
    }
    Ok(SocketAddressParts {
        host: host_port.to_string(),
        port: default_port,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn network_proxy_settings_default_matches_local_use_baseline() {
        assert_eq!(
            NetworkProxyConfig::default(),
            NetworkProxyConfig {
                enabled: false,
                proxy_url: "http://127.0.0.1:3128".to_string(),
                enable_socks5: true,
                socks_url: "http://127.0.0.1:8081".to_string(),
                enable_socks5_udp: true,
                allow_upstream_proxy: true,
                dangerously_allow_non_loopback_proxy: false,
                mode: NetworkMode::Full,
                domains: None,
                allow_local_binding: false,
                mitm: false,
                credential_broker: false,
                credential_broker_openai_host: None,
                dangerously_allow_plaintext_credential_injection: false,
                mitm_hooks: Vec::new(),
            }
        );
    }

    #[test]
    fn credential_broker_only_accepts_trusted_https_openai_endpoints() {
        let mut config = NetworkProxyConfig::default();

        for (base_url, expected_host) in [
            (
                Some("https://gateway.example.com/v1"),
                Some("gateway.example.com"),
            ),
            (
                Some("https://gateway.example.com./v1"),
                Some("gateway.example.com"),
            ),
            (Some("https://[2001:db8::1]/v1"), Some("2001:db8::1")),
            (Some("http://gateway.example.com/v1"), None),
            (Some("https://user@gateway.example.com/v1"), None),
            (Some("not-a-url"), None),
            (None, None),
        ] {
            config.set_credential_broker_openai_base_url(base_url);
            assert_eq!(
                config.credential_broker_openai_host.as_deref(),
                expected_host
            );
        }
    }

    #[test]
    fn managed_proxy_ports_reject_ephemeral_ports() {
        let mut config = NetworkProxyConfig {
            proxy_url: "http://127.0.0.1:0".to_string(),
            ..Default::default()
        };

        assert_eq!(
            managed_proxy_ports(&config).unwrap_err().to_string(),
            "network.proxy_url must use a fixed non-zero port for managed proxy provisioning"
        );

        config.proxy_url = "http://127.0.0.1:3128".to_string();
        config.socks_url = "socks5h://127.0.0.1:48081".to_string();
        assert_eq!(managed_proxy_ports(&config).unwrap(), vec![3128, 48081]);

        config.socks_url = "socks5h://127.0.0.1:0".to_string();
        assert_eq!(
            managed_proxy_ports(&config).unwrap_err().to_string(),
            "network.socks_url must use a fixed non-zero port for managed proxy provisioning"
        );

        config.enable_socks5 = false;
        assert_eq!(managed_proxy_ports(&config).unwrap(), vec![3128]);
    }

    #[test]
    fn network_proxy_config_uses_struct_defaults_for_missing_fields() {
        let config: NetworkProxyConfig = serde_json::from_str(r#"{ "enabled": true }"#).unwrap();
        let expected = NetworkProxyConfig {
            enabled: true,
            ..NetworkProxyConfig::default()
        };

        assert_eq!(config, expected);
    }

    #[test]
    fn set_allowed_domains_preserves_existing_deny_for_same_pattern() {
        let mut settings = NetworkProxyConfig::default();
        settings.set_denied_domains(vec!["example.com".to_string()]);

        settings.set_allowed_domains(vec!["example.com".to_string()]);

        assert_eq!(settings.allowed_domains(), None);
        assert_eq!(
            settings.denied_domains(),
            Some(vec!["example.com".to_string()])
        );
    }

    #[test]
    fn network_domain_permissions_serialize_to_effective_map_shape() {
        let mut settings = NetworkProxyConfig::default();
        settings.set_denied_domains(vec!["example.com".to_string()]);
        settings.set_allowed_domains(vec!["example.com".to_string()]);
        let config = settings;

        let value = serde_json::to_value(&config).unwrap();

        assert_eq!(
            value,
            serde_json::json!({
                "enabled": false,
                "proxy_url": "http://127.0.0.1:3128",
                "enable_socks5": true,
                "socks_url": "http://127.0.0.1:8081",
                "enable_socks5_udp": true,
                "allow_upstream_proxy": true,
                "dangerously_allow_non_loopback_proxy": false,
                "mode": "full",
                "domains": {
                    "example.com": "deny",
                },
                "allow_local_binding": false,
                "mitm": false,
                "credential_broker": false,
                "dangerously_allow_plaintext_credential_injection": false,
                "mitm_hooks": [],
            })
        );
    }

    #[test]
    fn parse_host_port_defaults_for_empty_string() {
        assert!(parse_host_port("", /*default_port*/ 1234).is_err());
    }

    #[test]
    fn parse_host_port_defaults_for_whitespace() {
        assert!(parse_host_port("   ", /*default_port*/ 5555).is_err());
    }

    #[test]
    fn parse_host_port_parses_host_port_without_scheme() {
        assert_eq!(
            parse_host_port("127.0.0.1:8080", /*default_port*/ 3128).unwrap(),
            SocketAddressParts {
                host: "127.0.0.1".to_string(),
                port: 8080,
            }
        );
    }

    #[test]
    fn parse_host_port_parses_host_port_with_scheme_and_path() {
        assert_eq!(
            parse_host_port(
                "http://example.com:8080/some/path",
                /*default_port*/ 3128
            )
            .unwrap(),
            SocketAddressParts {
                host: "example.com".to_string(),
                port: 8080,
            }
        );
    }

    #[test]
    fn parse_host_port_strips_userinfo() {
        assert_eq!(
            parse_host_port(
                "http://user:pass@host.example:5555",
                /*default_port*/ 3128
            )
            .unwrap(),
            SocketAddressParts {
                host: "host.example".to_string(),
                port: 5555,
            }
        );
    }

    #[test]
    fn parse_host_port_parses_ipv6_with_brackets() {
        assert_eq!(
            parse_host_port("http://[::1]:9999", /*default_port*/ 3128).unwrap(),
            SocketAddressParts {
                host: "::1".to_string(),
                port: 9999,
            }
        );
    }

    #[test]
    fn parse_host_port_does_not_treat_unbracketed_ipv6_as_host_port() {
        assert_eq!(
            parse_host_port("2001:db8::1", /*default_port*/ 3128).unwrap(),
            SocketAddressParts {
                host: "2001:db8::1".to_string(),
                port: 3128,
            }
        );
    }

    #[test]
    fn parse_host_port_falls_back_to_default_port_when_port_is_invalid() {
        assert_eq!(
            parse_host_port("example.com:notaport", /*default_port*/ 3128).unwrap(),
            SocketAddressParts {
                host: "example.com".to_string(),
                port: 3128,
            }
        );
    }

    #[test]
    fn host_and_port_from_network_addr_defaults_for_empty_string() {
        assert_eq!(
            host_and_port_from_network_addr("", /*default_port*/ 1234),
            "<missing>"
        );
    }

    #[test]
    fn host_and_port_from_network_addr_formats_ipv6() {
        assert_eq!(
            host_and_port_from_network_addr("http://[::1]:8080", /*default_port*/ 3128),
            "[::1]:8080"
        );
    }

    #[test]
    fn resolve_addr_maps_localhost_to_loopback() {
        assert_eq!(
            resolve_addr("localhost", /*default_port*/ 3128).unwrap(),
            "127.0.0.1:3128".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn resolve_addr_parses_ip_literals() {
        assert_eq!(
            resolve_addr("1.2.3.4", /*default_port*/ 80).unwrap(),
            "1.2.3.4:80".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn resolve_addr_parses_ipv6_literals() {
        assert_eq!(
            resolve_addr("http://[::1]:8080", /*default_port*/ 3128).unwrap(),
            "[::1]:8080".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn resolve_addr_falls_back_to_loopback_for_hostnames() {
        assert_eq!(
            resolve_addr("http://example.com:5555", /*default_port*/ 3128).unwrap(),
            "127.0.0.1:5555".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn clamp_bind_addrs_allows_non_loopback_when_enabled() {
        let cfg = NetworkProxyConfig {
            dangerously_allow_non_loopback_proxy: true,
            ..Default::default()
        };
        let http_addr = "0.0.0.0:3128".parse::<SocketAddr>().unwrap();
        let socks_addr = "0.0.0.0:8081".parse::<SocketAddr>().unwrap();

        let (http_addr, socks_addr) = clamp_bind_addrs(http_addr, socks_addr, &cfg);

        assert_eq!(http_addr, "0.0.0.0:3128".parse::<SocketAddr>().unwrap());
        assert_eq!(socks_addr, "0.0.0.0:8081".parse::<SocketAddr>().unwrap());
    }
}
