use super::ToolSuggestDiscoverablePlugin;
use super::ToolSuggestPluginDiscoveryInput;
use crate::PluginInstallRequest;
use crate::PluginsConfigInput;
use crate::PluginsManager;
use crate::startup_sync::curated_plugins_repo_path;
use crate::test_support::TEST_CURATED_PLUGIN_SHA;
use crate::test_support::load_plugins_config;
use crate::test_support::test_plugins_manager;
use crate::test_support::test_plugins_manager_with_options;
use crate::test_support::write_curated_plugin;
use crate::test_support::write_curated_plugin_sha_with;
use crate::test_support::write_file;
use crate::test_support::write_openai_api_curated_marketplace;
use crate::test_support::write_openai_curated_marketplace;
use codex_config::CONFIG_TOML_FILE;
use codex_login::CodexAuth;
use codex_protocol::auth::AuthMode;
use codex_protocol::protocol::Product;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;
use std::collections::HashSet;
use std::path::Path;
use tempfile::tempdir;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_test::internal::MockWriter;


#[tokio::test]
async fn returns_api_curated_fallback_plugins_for_direct_provider_auth() {
    let codex_home = tempdir().expect("tempdir should succeed");
    let curated_root = curated_plugins_repo_path(codex_home.path());
    write_openai_api_curated_marketplace(&curated_root, &["sample", "slack", "openai-developers"]);

    let plugins = load_plugins_config(codex_home.path(), codex_home.path()).await;
    let plugins_manager = test_plugins_manager_with_options(
        codex_home.path().to_path_buf(),
        Some(Product::Codex),
        Some(AuthMode::ApiKey),
    );
    let auth = CodexAuth::from_api_key("test-api-key");
    let discoverable_plugins = list_discoverable_plugins(
        &plugins_manager,
        discovery_input(plugins, &[], &[], &[]),
        Some(&auth),
    )
    .await;

    assert_eq!(
        discoverable_plugins
            .into_iter()
            .map(|plugin| plugin.id)
            .collect::<Vec<_>>(),
        vec![
            "openai-developers@openai-api-curated".to_string(),
            "slack@openai-api-curated".to_string(),
        ]
    );
}



#[tokio::test]
async fn includes_openai_api_curated_when_remote_enabled_without_auth() {
    let codex_home = tempdir().expect("tempdir should succeed");
    let curated_root = curated_plugins_repo_path(codex_home.path());
    write_openai_api_curated_marketplace(&curated_root, &["slack"]);

    let plugins = load_plugins_config(codex_home.path(), codex_home.path()).await;
    let plugins_manager = test_plugins_manager(codex_home.path().to_path_buf());
    let discoverable_plugins = list_discoverable_plugins(
        &plugins_manager,
        discovery_input(plugins, &[], &[], &[]),
        /*auth*/ None,
    )
    .await;

    assert_eq!(
        discoverable_plugins
            .into_iter()
            .map(|plugin| plugin.id)
            .collect::<Vec<_>>(),
        vec!["slack@openai-api-curated".to_string()]
    );
}







#[tokio::test]
async fn omits_installed_curated_plugins() {
    let codex_home = tempdir().expect("tempdir should succeed");
    let curated_root = curated_plugins_repo_path(codex_home.path());
    write_openai_curated_marketplace(&curated_root, &["slack"]);
    install_marketplace_plugin(codex_home.path(), curated_root.as_path(), "slack").await;

    let plugins = load_plugins_config(codex_home.path(), codex_home.path()).await;
    let plugins_manager = test_plugins_manager(codex_home.path().to_path_buf());
    let discoverable_plugins = list_discoverable_plugins(
        &plugins_manager,
        discovery_input(plugins, &[], &[], &[]),
        /*auth*/ None,
    )
    .await;

    assert_eq!(discoverable_plugins, Vec::new());
}



#[tokio::test]
async fn does_not_expand_local_plugins_by_installed_apps() {
    let codex_home = tempdir().expect("tempdir should succeed");
    let curated_root = curated_plugins_repo_path(codex_home.path());
    write_openai_curated_marketplace(&curated_root, &["sample", "slack", "hubspot"]);
    write_plugin_app(&curated_root, "sample", "sample", "connector_sample");
    install_marketplace_plugin(codex_home.path(), curated_root.as_path(), "slack").await;

    let plugins = load_plugins_config(codex_home.path(), codex_home.path()).await;
    let plugins_manager = test_plugins_manager(codex_home.path().to_path_buf());
    let discoverable_plugins = list_discoverable_plugins(
        &plugins_manager,
        discovery_input(plugins, &[], &[], &[]),
        /*auth*/ None,
    )
    .await;

    assert_eq!(discoverable_plugins, Vec::new());
}

#[tokio::test]
async fn does_not_read_local_plugins_for_loaded_apps() {
    let hubspot_app_id = "asdk_app_697acb8e53d88191bf7a79e62012ae14";
    let granola_app_id = "asdk_app_697761cab6f48191b5ed345919a3ce8b";
    let codex_home = tempdir().expect("tempdir should succeed");
    let curated_root = curated_plugins_repo_path(codex_home.path());
    write_openai_curated_marketplace(&curated_root, &["hubspot", "granola", "sample"]);
    write_plugin_app(&curated_root, "hubspot", "hubspot", hubspot_app_id);
    write_plugin_app(&curated_root, "granola", "granola", granola_app_id);
    write_file(
        &curated_root.join("plugins/sample/.app.json"),
        "invalid json",
    );

    let plugins = load_plugins_config(codex_home.path(), codex_home.path()).await;
    let plugins_manager = test_plugins_manager(codex_home.path().to_path_buf());
    let buffer: &'static std::sync::Mutex<Vec<u8>> =
        Box::leak(Box::new(std::sync::Mutex::new(Vec::new())));
    let subscriber = tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(false)
        .with_max_level(Level::WARN)
        .with_span_events(FmtSpan::NONE)
        .with_writer(MockWriter::new(buffer))
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    let discoverable_plugins = list_discoverable_plugins(
        &plugins_manager,
        discovery_input(plugins, &[], &[], &[hubspot_app_id]),
        /*auth*/ None,
    )
    .await;

    assert_eq!(discoverable_plugins, Vec::new());
    let logs = String::from_utf8(buffer.lock().expect("buffer lock").clone())
        .expect("utf8 logs")
        .replace('\\', "/");
    assert_eq!(logs.matches("plugins/sample/.app.json").count(), 0);
}

#[tokio::test]
async fn does_not_expand_local_sales_apps() {
    let hubspot_app_id = "asdk_app_697acb8e53d88191bf7a79e62012ae14";
    let granola_app_id = "asdk_app_697761cab6f48191b5ed345919a3ce8b";
    let test_app_id = "asdk_app_test_source";
    let codex_home = tempdir().expect("tempdir should succeed");
    let curated_root = curated_plugins_repo_path(codex_home.path());
    write_openai_curated_marketplace(&curated_root, &["hubspot", "granola", "test-source"]);
    write_plugin_app(&curated_root, "hubspot", "hubspot", hubspot_app_id);
    write_plugin_app(&curated_root, "granola", "granola", granola_app_id);
    write_plugin_app(&curated_root, "test-source", "test_source", test_app_id);

    let sales_marketplace_name = "oai-maintained-plugins";
    let sales_marketplace_root = codex_home
        .path()
        .join(format!(".tmp/marketplaces/{sales_marketplace_name}"));
    write_file(
        &sales_marketplace_root.join(".agents/plugins/marketplace.json"),
        &format!(
            r#"{{
  "name": "{sales_marketplace_name}",
  "plugins": [
    {{"name": "sales", "source": {{"source": "local", "path": "./plugins/sales"}}}}
  ]
}}
"#
        ),
    );
    write_curated_plugin(&sales_marketplace_root, "sales");
    write_file(
        &sales_marketplace_root.join("plugins/sales/.app.json"),
        &format!(
            r#"{{
  "apps": {{
    "hubspot": {{
      "id": "{hubspot_app_id}"
    }},
    "granola": {{
      "id": "{granola_app_id}"
    }}
  }}
}}
"#
        ),
    );
    write_file(
        &codex_home.path().join(CONFIG_TOML_FILE),
        &format!(
            r#"[features]
plugins = true

[marketplaces.{sales_marketplace_name}]
source_type = "git"
source = "/tmp/{sales_marketplace_name}"
"#
        ),
    );
    install_marketplace_plugin(codex_home.path(), sales_marketplace_root.as_path(), "sales").await;

    let plugins = load_plugins_config(codex_home.path(), codex_home.path()).await;
    let plugins_manager = test_plugins_manager(codex_home.path().to_path_buf());
    let discoverable_plugins = list_discoverable_plugins(
        &plugins_manager,
        discovery_input(plugins, &[], &[], &[]),
        /*auth*/ None,
    )
    .await;

    assert_eq!(discoverable_plugins, Vec::new());
}


fn discovery_input(
    plugins: PluginsConfigInput,
    configured_plugin_ids: &[&str],
    disabled_plugin_ids: &[&str],
    loaded_plugin_app_connector_ids: &[&str],
) -> ToolSuggestPluginDiscoveryInput {
    ToolSuggestPluginDiscoveryInput {
        plugins,
        configured_plugin_ids: string_set(configured_plugin_ids),
        disabled_plugin_ids: string_set(disabled_plugin_ids),
        loaded_plugin_app_connector_ids: string_set(loaded_plugin_app_connector_ids),
    }
}

async fn list_discoverable_plugins(
    plugins_manager: &PluginsManager,
    input: ToolSuggestPluginDiscoveryInput,
    auth: Option<&CodexAuth>,
) -> Vec<ToolSuggestDiscoverablePlugin> {
    plugins_manager
        .list_tool_suggest_discoverable_plugins(&input, auth)
        .await
        .expect("discoverable plugins should load")
}

fn string_set(values: &[&str]) -> HashSet<String> {
    values.iter().map(ToString::to_string).collect()
}

async fn install_marketplace_plugin(codex_home: &Path, marketplace_root: &Path, plugin_name: &str) {
    write_curated_plugin_sha_with(codex_home, TEST_CURATED_PLUGIN_SHA);
    let config = load_plugins_config(codex_home, marketplace_root).await;
    test_plugins_manager(codex_home.to_path_buf())
        .install_plugin(
            &config.config_layer_stack,
            PluginInstallRequest {
                plugin_name: plugin_name.to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    marketplace_root.join(".agents/plugins/marketplace.json"),
                )
                .expect("marketplace path"),
            },
        )
        .await
        .expect("plugin should install");
}

fn write_plugin_app(root: &Path, plugin_name: &str, app_name: &str, app_id: &str) {
    write_file(
        &root.join(format!("plugins/{plugin_name}/.app.json")),
        &format!(
            r#"{{
  "apps": {{
    "{app_name}": {{
      "id": "{app_id}"
    }}
  }}
}}
"#
        ),
    );
}
