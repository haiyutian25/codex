use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

use codex_network_proxy::NetworkProxyConfig;
use codex_network_proxy::PROXY_ATTRIBUTION_TOKEN_ENV_KEY;
use codex_network_proxy::RemoteNetworkProxyConfig;
use codex_network_proxy::RemoteNetworkProxyLaunchConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;

use super::prepare_exec_request;
use crate::ExecParams;
use crate::ProcessId;

#[tokio::test]
async fn native_request_preserves_native_launch_fields() {
    let cwd: AbsolutePathBuf = std::env::current_dir()
        .expect("current directory")
        .try_into()
        .expect("absolute cwd");
    let cwd_uri = PathUri::from_abs_path(&cwd);
    let env = HashMap::from([("TEST_ENV".to_string(), "value".to_string())]);
    let params = ExecParams {
        process_id: ProcessId::from("process-1"),
        argv: vec!["echo".to_string(), "hello".to_string()],
        cwd: cwd_uri,
        shell_snapshot: None,
        env_policy: None,
        env: HashMap::new(),
        tty: false,
        pipe_stdin: false,
        arg0: Some("custom-arg0".to_string()),
        sandbox: None,
        enforce_managed_network: false,
        managed_network: None,
        network_proxy: None,
    };

    let prepared = prepare_exec_request(
        &params,
        env.clone(),
        /*runtime_paths*/ None,
        /*network_policy_decider*/ None,
        /*network_policy_audit_observer*/ None,
    )
    .await
    .expect("prepare native request");

    assert_eq!(prepared.command, params.argv);
    assert_eq!(prepared.cwd, cwd);
    assert_eq!(prepared.env, env);
    assert_eq!(prepared.arg0, params.arg0);
}

// Executor-local proxy launch path.
#[tokio::test]
async fn native_request_handles_remote_proxy_config_for_platform() {
    let cwd: AbsolutePathBuf = std::env::current_dir()
        .expect("current directory")
        .try_into()
        .expect("absolute cwd");
    let mut config = NetworkProxyConfig {
        enabled: true,
        ..NetworkProxyConfig::default()
    };
    config.set_allowed_domains(vec!["allowed.example".to_string()]);
    let proxy_config = RemoteNetworkProxyConfig::from_effective_config(&config)
        .expect("supported remote proxy config");
    let params = ExecParams {
        process_id: ProcessId::from("process-remote-proxy"),
        argv: vec!["echo".to_string(), "hello".to_string()],
        cwd: PathUri::from_abs_path(&cwd),
        shell_snapshot: None,
        env_policy: None,
        env: HashMap::new(),
        tty: false,
        pipe_stdin: false,
        arg0: None,
        sandbox: None,
        enforce_managed_network: false,
        managed_network: None,
        network_proxy: Some(
            RemoteNetworkProxyLaunchConfig::new(proxy_config)
                .for_execution("remote".to_string(), "execution-1".to_string()),
        ),
    };
    let stale_proxy = "http://127.0.0.1:9".to_string();
    let env = HashMap::from([
        ("HTTP_PROXY".to_string(), stale_proxy.clone()),
        ("TEST_ENV".to_string(), "value".to_string()),
        (
            PROXY_ATTRIBUTION_TOKEN_ENV_KEY.to_string(),
            "foreign-token".to_string(),
        ),
    ]);

    let prepared = prepare_exec_request(
        &params, env, /*runtime_paths*/ None, /*network_policy_decider*/ None,
        /*network_policy_audit_observer*/ None,
    )
    .await
    .expect("prepare request with executor-local proxy");

    let http_proxy = prepared.env.get("HTTP_PROXY").expect("HTTP proxy env");
    assert_ne!(http_proxy, &stale_proxy);
    assert!(http_proxy.starts_with("http://127.0.0.1:"));
    assert!(!prepared.env.contains_key(PROXY_ATTRIBUTION_TOKEN_ENV_KEY));
    let proxy_addr: SocketAddr = http_proxy
        .strip_prefix("http://")
        .expect("HTTP proxy scheme")
        .parse()
        .expect("HTTP proxy address");
    let mut stream = tokio::net::TcpStream::connect(proxy_addr)
        .await
        .expect("connect to executor proxy");
    stream
        .write_all(b"CONNECT blocked.example:443 HTTP/1.1\r\nHost: blocked.example:443\r\n\r\n")
        .await
        .expect("write CONNECT request");
    let mut response = [0_u8; 256];
    let response_len = timeout(Duration::from_secs(2), stream.read(&mut response))
        .await
        .expect("proxy response timeout")
        .expect("read proxy response");
    assert!(String::from_utf8_lossy(&response[..response_len]).starts_with("HTTP/1.1 403"));

    prepared
        .network_proxy_handle
        .expect("running executor proxy")
        .shutdown()
        .await
        .expect("shut down executor proxy");
}

#[tokio::test]
async fn disabled_remote_proxy_config_is_rejected_before_exporting_ports() {
    let cwd: AbsolutePathBuf = std::env::current_dir()
        .expect("current directory")
        .try_into()
        .expect("absolute cwd");
    let proxy_config =
        RemoteNetworkProxyConfig::from_effective_config(&NetworkProxyConfig::default())
            .expect("serializable disabled proxy config");
    let params = ExecParams {
        process_id: ProcessId::from("process-disabled-remote-proxy"),
        argv: vec!["echo".to_string(), "hello".to_string()],
        cwd: PathUri::from_abs_path(&cwd),
        shell_snapshot: None,
        env_policy: None,
        env: HashMap::new(),
        tty: false,
        pipe_stdin: false,
        arg0: None,
        sandbox: None,
        enforce_managed_network: false,
        managed_network: None,
        network_proxy: Some(RemoteNetworkProxyLaunchConfig::new(proxy_config)),
    };

    let error = prepare_exec_request(
        &params,
        HashMap::new(),
        /*runtime_paths*/ None,
        /*network_policy_decider*/ None,
        /*network_policy_audit_observer*/ None,
    )
    .await
    .err()
    .expect("disabled executor proxy launch must fail closed");

    assert_eq!(error.code, -32602);
    assert!(
        error
            .message
            .contains("executor-local network proxy launch requires an enabled proxy")
    );
}

