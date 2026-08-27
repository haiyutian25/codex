use super::*;
use crate::LoadedPlugin;
use crate::OPENAI_CURATED_MARKETPLACE_NAME;
use crate::PluginLoadOutcome;
use crate::loader::load_plugin_skill_inventory;
use crate::marketplace::MarketplacePluginInstallPolicy;
use crate::remote::REMOTE_GLOBAL_MARKETPLACE_NAME;
use crate::remote::REMOTE_WORKSPACE_MARKETPLACE_NAME;
use crate::remote::REMOTE_WORKSPACE_SHARED_WITH_ME_MARKETPLACE_NAME;
use crate::remote::RemoteInstalledPlugin;
use crate::startup_sync::curated_plugins_repo_path;
use crate::test_support::TEST_CURATED_PLUGIN_CACHE_VERSION;
use crate::test_support::TEST_CURATED_PLUGIN_SHA;
use crate::test_support::load_plugins_config as load_plugins_config_input;
use crate::test_support::test_http_client_factory;
use crate::test_support::test_plugins_manager;
use crate::test_support::test_plugins_manager_with_options;
use crate::test_support::test_skill_root_loader;
use crate::test_support::write_curated_plugin_sha_with as write_curated_plugin_sha;
use crate::test_support::write_file;
use crate::test_support::write_openai_curated_marketplace;
use codex_config::AppToolApproval;
use codex_config::CONFIG_TOML_FILE;
use codex_config::ConfigLayerEntry;
use codex_config::ConfigLayerSource;
use codex_config::ConfigLayerStack;
use codex_config::ConfigRequirements;
use codex_config::ConfigRequirementsToml;
use codex_config::McpServerConfig;
use codex_config::McpServerToolConfig;
use codex_config::RequirementSource;
use codex_config::RequirementsLayerEntry;
use codex_config::SkillConfigRules;
use codex_config::compose_requirements;
use codex_config::types::McpServerTransportConfig;
use codex_plugin::AppDeclaration;
use codex_plugin::PluginId;
use codex_protocol::auth::AuthMode;
use codex_protocol::protocol::Product;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_absolute_path::test_support::PathBufExt;
use codex_utils_plugins::SkillDiscoveryMode;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use toml::Value;

const MAX_CAPABILITY_SUMMARY_DESCRIPTION_LEN: usize = 1024;

fn unrestricted_config_layer_stack() -> ConfigLayerStack {
    ConfigLayerStack::default()
}

fn config_layer_stack_with_requirements(
    codex_home: &Path,
    user_config: &str,
    requirements: &str,
) -> ConfigLayerStack {
    let with_sources = compose_requirements([RequirementsLayerEntry::from_toml(
        RequirementSource::Unknown,
        requirements,
    )])
    .expect("compose requirements")
    .expect("requirements should be present");
    let requirements_toml = with_sources.clone().into_toml();
    let requirements = ConfigRequirements::try_from(with_sources).expect("normalize requirements");
    let config_file =
        AbsolutePathBuf::try_from(codex_home.join(CONFIG_TOML_FILE)).expect("absolute config path");
    ConfigLayerStack::new(
        vec![ConfigLayerEntry::new(
            ConfigLayerSource::User {
                file: config_file,
                profile: None,
            },
            toml::from_str(user_config).expect("parse user config"),
        )],
        requirements,
        requirements_toml,
    )
    .expect("build config layer stack")
}

fn plugins_config_input_with_requirements(
    codex_home: &Path,
    user_config: &str,
    requirements: &str,
) -> PluginsConfigInput {
    PluginsConfigInput::new(
        config_layer_stack_with_requirements(codex_home, user_config, requirements),
        String::new(),
        /*plugins_enabled*/ true,
        /*remote_plugin_enabled*/ false,
        String::new(),
        test_http_client_factory(),
    )
}



#[test]
fn marketplace_source_refresh_notifies_only_after_installed_cache_changes() {
    let tmp = TempDir::new().unwrap();
    let manager = test_plugins_manager(tmp.path().to_path_buf());
    let callback_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let callback_count_for_callback = Arc::clone(&callback_count);
    let callback: EffectivePluginsChangedCallback = Arc::new(move |_change| {
        callback_count_for_callback.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    });

    manager.clear_caches_after_marketplace_source_refresh(
        /*installed_plugin_cache_refreshed*/ false,
        Some(&callback),
    );
    assert_eq!(callback_count.load(std::sync::atomic::Ordering::Relaxed), 0);

    manager.clear_caches_after_marketplace_source_refresh(
        /*installed_plugin_cache_refreshed*/ true,
        Some(&callback),
    );
    assert_eq!(callback_count.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[tokio::test]
async fn marketplace_policy_projection_disables_installed_plugin_and_invalidates_cache() {
    let codex_home = TempDir::new().expect("create Codex home");
    write_plugin(
        &codex_home.path().join("plugins/cache/company"),
        "sample/local",
        "sample",
    );
    let user_config = r#"
[marketplaces.company]
source_type = "git"
source = "https://github.com/example/company.git"

[plugins."sample@company"]
enabled = true
"#;
    let allowed = plugins_config_input_with_requirements(
        codex_home.path(),
        user_config,
        r#"
[marketplaces]
restrict_to_allowed_sources = true

[marketplaces.allowed_sources.company]
source = "git"
url = "https://github.com/example/company.git"
"#,
    );
    let blocked = plugins_config_input_with_requirements(
        codex_home.path(),
        user_config,
        r#"
[marketplaces]
restrict_to_allowed_sources = true

[marketplaces.allowed_sources.other]
source = "git"
url = "https://github.com/example/other.git"
"#,
    );
    let manager = test_plugins_manager(codex_home.path().to_path_buf());

    let allowed_outcome = manager.plugins_for_config(&allowed).await;
    assert_eq!(allowed_outcome.plugins().len(), 1);
    assert_eq!(allowed_outcome.plugins()[0].config_name, "sample@company");

    let blocked_outcome = manager.plugins_for_config(&blocked).await;
    assert_eq!(blocked_outcome, PluginLoadOutcome::default());
}

#[tokio::test]
async fn plugin_read_rejects_marketplace_blocked_by_requirements() {
    let codex_home = TempDir::new().expect("create Codex home");
    let marketplace_root = codex_home.path().join("marketplace");
    write_plugin(&marketplace_root, "sample", "sample");
    write_file(
        &marketplace_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "company",
  "plugins": [
    {
      "name": "sample",
      "source": {"source": "local", "path": "./sample"}
    }
  ]
}"#,
    );
    let config = plugins_config_input_with_requirements(
        codex_home.path(),
        "",
        r#"
[marketplaces]
restrict_to_allowed_sources = true
"#,
    );
    let marketplace_path =
        AbsolutePathBuf::try_from(marketplace_root.join(".agents/plugins/marketplace.json"))
            .expect("absolute marketplace path");

    let err = test_plugins_manager(codex_home.path().to_path_buf())
        .read_plugin_for_config(
            &config,
            &PluginReadRequest {
                plugin_name: "sample".to_string(),
                marketplace_path,
            },
        )
        .await
        .expect_err("blocked marketplace should not be readable");
    assert!(matches!(
        err,
        MarketplaceError::InvalidMarketplaceFile { .. }
    ));
}

#[test]
fn marketplace_policy_filters_discovered_marketplaces_by_configured_name() {
    let codex_home = TempDir::new().expect("create Codex home");
    let repo_root = codex_home.path().join("repo");
    let subdirectory = repo_root.join("worktree/subdirectory");
    fs::create_dir_all(&subdirectory).expect("create input subdirectory");
    write_plugin(&repo_root, "sample", "sample");
    write_file(
        &repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "company",
  "plugins": [
    {
      "name": "sample",
      "source": {"source": "local", "path": "./sample"}
    }
  ]
}"#,
    );
    init_git_repo(&repo_root);
    let repo_root = AbsolutePathBuf::try_from(repo_root).expect("absolute repository root");
    let subdirectory =
        AbsolutePathBuf::try_from(subdirectory).expect("absolute input subdirectory");
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let user_config = format!(
        r#"
[marketplaces.company]
source_type = "local"
source = {:?}
"#,
        repo_root.as_path()
    );
    let allowed = plugins_config_input_with_requirements(
        codex_home.path(),
        &user_config,
        &format!(
            r#"
[marketplaces]
restrict_to_allowed_sources = true

[marketplaces.allowed_sources.company]
source = "local"
path = {:?}
"#,
            repo_root.as_path()
        ),
    );
    let blocked = plugins_config_input_with_requirements(
        codex_home.path(),
        &user_config,
        &format!(
            r#"
[marketplaces]
restrict_to_allowed_sources = true

[marketplaces.allowed_sources.subdirectory]
source = "local"
path = {:?}
"#,
            subdirectory.as_path()
        ),
    );

    let allowed_outcome = manager
        .list_marketplaces_for_config(
            &allowed,
            std::slice::from_ref(&subdirectory),
            /*include_openai_curated*/ false,
        )
        .expect("list allowed marketplace");
    assert_eq!(allowed_outcome.marketplaces.len(), 1);
    assert_eq!(allowed_outcome.marketplaces[0].name, "company");

    let blocked_outcome = manager
        .list_marketplaces_for_config(
            &blocked,
            std::slice::from_ref(&subdirectory),
            /*include_openai_curated*/ false,
        )
        .expect("list blocked marketplace");
    assert_eq!(blocked_outcome.marketplaces, Vec::new());
}

fn write_auth_projection_plugin(codex_home: &Path, name: &str, include_app: bool) {
    let plugin_root = codex_home
        .join("plugins/cache")
        .join("test")
        .join(name)
        .join("local");
    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        &format!(r#"{{"name":"{name}"}}"#),
    );
    write_file(
        &plugin_root.join(".mcp.json"),
        &format!(
            r#"{{
  "mcpServers": {{
    "{name}": {{
      "type": "stdio",
      "command": "{name}-mcp"
    }}
  }}
}}"#
        ),
    );
    if include_app {
        write_auth_projection_app(codex_home, name, name);
    }
}

fn write_auth_projection_app(codex_home: &Path, plugin_name: &str, app_name: &str) {
    let plugin_root = codex_home
        .join("plugins/cache")
        .join("test")
        .join(plugin_name)
        .join("local");
    write_file(
        &plugin_root.join(".app.json"),
        &format!(r#"{{"apps":{{"{app_name}":{{"id":"connector_{plugin_name}"}}}}}}"#),
    );
}

fn app_declaration(name: &str, connector_id: &str) -> AppDeclaration {
    AppDeclaration {
        name: name.to_string(),
        connector_id: AppConnectorId(connector_id.to_string()),
        category: None,
    }
}

async fn auth_projection_config(codex_home: &Path) -> PluginsConfigInput {
    let config_toml = r#"[features]
plugins = true

[plugins."sample@test"]
enabled = true

[plugins."docs@test"]
enabled = true
"#
    .to_string();
    write_file(&codex_home.join(CONFIG_TOML_FILE), &config_toml);
    load_config(codex_home, codex_home).await
}

fn sorted_effective_mcp_server_names(outcome: &PluginLoadOutcome) -> Vec<String> {
    let mut names = outcome
        .effective_mcp_servers()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    names.sort();
    names
}

#[tokio::test]
async fn plugin_auth_projection_hides_apps_without_chatgpt_auth() {
    let codex_home = TempDir::new().unwrap();
    write_auth_projection_plugin(codex_home.path(), "sample", /*include_app*/ true);
    write_auth_projection_plugin(codex_home.path(), "docs", /*include_app*/ false);
    let config = auth_projection_config(codex_home.path()).await;
    let manager = test_plugins_manager_with_options(
        codex_home.path().to_path_buf(),
        Some(Product::Codex),
        Some(AuthMode::ApiKey),
    );

    let outcome = manager.plugins_for_config(&config).await;

    assert!(outcome.effective_apps().is_empty());
    assert_eq!(
        sorted_effective_mcp_server_names(&outcome),
        vec!["docs".to_string(), "sample".to_string()]
    );
    let sample = outcome
        .capability_summaries()
        .iter()
        .find(|plugin| plugin.config_name == "sample@test")
        .expect("sample plugin summary should exist");
    assert_eq!(sample.mcp_server_names, vec!["sample".to_string()]);
    assert!(sample.app_connector_ids.is_empty());
}





fn write_plugin_with_version(
    root: &Path,
    dir_name: &str,
    manifest_name: &str,
    manifest_version: Option<&str>,
) {
    let plugin_root = root.join(dir_name);
    fs::create_dir_all(plugin_root.join(".codex-plugin")).unwrap();
    fs::create_dir_all(plugin_root.join("skills")).unwrap();
    let version = manifest_version
        .map(|manifest_version| format!(r#","version":"{manifest_version}""#))
        .unwrap_or_default();
    fs::write(
        plugin_root.join(".codex-plugin/plugin.json"),
        format!(r#"{{"name":"{manifest_name}"{version}}}"#),
    )
    .unwrap();
    fs::write(
        plugin_root.join("skills/SKILL.md"),
        format!("---\nname: {manifest_name}-skill\ndescription: test skill\n---\n\n# Test skill\n"),
    )
    .unwrap();
    fs::write(plugin_root.join(".mcp.json"), r#"{"mcpServers":{}}"#).unwrap();
}

fn write_plugin(root: &Path, dir_name: &str, manifest_name: &str) {
    write_plugin_with_version(
        root,
        dir_name,
        manifest_name,
        /*manifest_version*/ None,
    );
}

fn init_git_repo(repo: &Path) {
    run_git(repo, &["init"]);
    run_git(repo, &["config", "user.email", "codex-test@example.com"]);
    run_git(repo, &["config", "user.name", "Codex Test"]);
    run_git(repo, &["add", "."]);
    run_git(repo, &["commit", "-m", "initial"]);
}

fn run_git(repo: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("git should run: {err}"));
    assert!(
        output.status.success(),
        "git -C {} {} failed\nstdout:\n{}\nstderr:\n{}",
        repo.display(),
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn plugin_config_toml(enabled: bool, plugins_feature_enabled: bool) -> String {
    let mut root = toml::map::Map::new();

    let mut features = toml::map::Map::new();
    features.insert(
        "plugins".to_string(),
        Value::Boolean(plugins_feature_enabled),
    );
    root.insert("features".to_string(), Value::Table(features));

    let mut plugin = toml::map::Map::new();
    plugin.insert("enabled".to_string(), Value::Boolean(enabled));

    let mut plugins = toml::map::Map::new();
    plugins.insert("sample@test".to_string(), Value::Table(plugin));
    root.insert("plugins".to_string(), Value::Table(plugins));

    toml::to_string(&Value::Table(root)).expect("plugin test config should serialize")
}

async fn load_plugins_from_config(
    config_toml: &str,
    codex_home: &Path,
    auth_mode: Option<AuthMode>,
) -> PluginLoadOutcome {
    write_file(&codex_home.join(CONFIG_TOML_FILE), config_toml);
    let config = load_config(codex_home, codex_home).await;
    test_plugins_manager_with_options(codex_home.to_path_buf(), Some(Product::Codex), auth_mode)
        .plugins_for_config(&config)
        .await
}

async fn load_config(codex_home: &Path, cwd: &Path) -> PluginsConfigInput {
    load_plugins_config_input(codex_home, cwd).await
}

fn remote_installed_linear_plugin() -> RemoteInstalledPlugin {
    remote_installed_plugin("linear")
}

fn remote_installed_plugin(name: &str) -> RemoteInstalledPlugin {
    remote_installed_plugin_in_marketplace(name, REMOTE_GLOBAL_MARKETPLACE_NAME)
}

fn remote_installed_plugin_in_marketplace(
    name: &str,
    marketplace_name: &str,
) -> RemoteInstalledPlugin {
    RemoteInstalledPlugin {
        marketplace_name: marketplace_name.to_string(),
        id: format!("plugins~Plugin_{name}"),
        version: None,
        name: name.to_string(),
        installed_at: None,
        enabled: true,
        install_policy: codex_app_server_protocol::PluginInstallPolicy::Available,
        install_policy_source: None,
        must_show_installation_interstitial: None,
        auth_policy: codex_app_server_protocol::PluginAuthPolicy::OnUse,
        availability: codex_app_server_protocol::PluginAvailability::Available,
        disabled_reason: None,
        eligible_plan_types: None,
        interface: None,
        keywords: Vec::new(),
    }
}

fn write_cached_plugin(codex_home: &Path, marketplace_name: &str, plugin_name: &str) {
    write_plugin_with_version(
        &codex_home
            .join("plugins/cache")
            .join(marketplace_name)
            .join(plugin_name),
        "local",
        plugin_name,
        /*manifest_version*/ Some("local"),
    );
}

#[tokio::test]
async fn load_plugins_loads_manifest_mcp_server_objects() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/counter-sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{
  "name": "counter-sample",
  "version": "1.1.1",
  "description": "Plugin that declares MCP servers in the manifest",
  "mcpServers": {
    "counter": {
      "type": "http",
      "url": "https://sample.example/counter/mcp"
    }
  }
}"#,
    );

    let config_toml = r#"
[features]
plugins = true

[plugins."counter-sample@test"]
enabled = true
"#;
    let outcome =
        load_plugins_from_config(config_toml, codex_home.path(), /*auth_mode*/ None).await;

    assert_eq!(outcome.plugins()[0].error, None);
    assert_eq!(
        outcome.plugins()[0].mcp_servers,
        HashMap::from([(
            "counter".to_string(),
            McpServerConfig {
                auth: Default::default(),
                transport: McpServerTransportConfig::StreamableHttp {
                    url: "https://sample.example/counter/mcp".to_string(),
                    bearer_token_env_var: None,
                    http_headers: None,
                    env_http_headers: None,
                    http_headers_helper: None,
                },
                environment_id: "local".to_string(),
                enabled: true,
                required: false,
                supports_parallel_tool_calls: false,
                omit_tools_from: None,
                disabled_reason: None,
                startup_timeout_sec: None,
                tool_timeout_sec: None,
                default_tools_approval_mode: None,
                enabled_tools: None,
                disabled_tools: None,
                scopes: None,
                oauth: None,
                oauth_resource: None,
                tools: HashMap::new(),
            },
        )])
    );
}

#[tokio::test]
async fn load_plugins_applies_plugin_mcp_server_policy() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{
  "name": "sample"
}"#,
    );
    write_file(
        &plugin_root.join(".mcp.json"),
        r#"{
  "mcpServers": {
    "sample": {
      "type": "http",
      "url": "https://sample.example/mcp",
      "default_tools_approval_mode": "prompt",
      "enabled_tools": ["read", "search"],
      "tools": {
        "search": { "approval_mode": "prompt" }
      }
    }
  }
}"#,
    );
    let config_toml = r#"
[features]
plugins = true

[plugins."sample@test"]
enabled = true

[plugins."sample@test".mcp_servers.sample]
enabled = false
default_tools_approval_mode = "approve"
enabled_tools = ["search"]
disabled_tools = ["delete"]

[plugins."sample@test".mcp_servers.sample.tools.search]
approval_mode = "approve"
"#;

    let outcome =
        load_plugins_from_config(config_toml, codex_home.path(), /*auth_mode*/ None).await;
    let server = outcome.plugins()[0]
        .mcp_servers
        .get("sample")
        .expect("sample server");

    assert!(!server.enabled);
    assert_eq!(
        server.default_tools_approval_mode,
        Some(AppToolApproval::Approve)
    );
    assert_eq!(server.enabled_tools, Some(vec!["search".to_string()]));
    assert_eq!(server.disabled_tools, Some(vec!["delete".to_string()]));
    assert_eq!(
        server.tools.get("search"),
        Some(&McpServerToolConfig {
            approval_mode: Some(AppToolApproval::Approve),
        })
    );
}


#[tokio::test]
async fn remote_installed_cache_ignores_plugins_missing_local_cache() {
    let codex_home = TempDir::new().unwrap();
    write_file(
        &codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features]
plugins = true
"#,
    );

    let config = load_config(codex_home.path(), codex_home.path()).await;
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    manager.write_remote_installed_plugins_cache(vec![remote_installed_linear_plugin()]);

    let outcome = manager.plugins_for_config(&config).await;
    assert_eq!(outcome, PluginLoadOutcome::default());
}

#[tokio::test]
async fn installed_plugin_telemetry_metadata_collects_capabilities() {
    let codex_home = TempDir::new().unwrap();
    write_cached_plugin(codex_home.path(), "test", "sample");
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let plugin_id = PluginId::parse("sample@test").expect("plugin id should parse");

    let metadata = manager
        .telemetry_metadata_for_installed_plugin(&plugin_id)
        .await;

    assert_eq!(
        metadata,
        PluginTelemetryMetadata {
            plugin_id: Some(plugin_id),
            remote_plugin_id: None,
            capability_summary: Some(PluginCapabilitySummary {
                config_name: "sample@test".to_string(),
                display_name: "sample".to_string(),
                plugin_namespace: Some("sample".to_string()),
                description: None,
                has_skills: true,
                mcp_server_names: Vec::new(),
                app_connector_ids: Vec::new(),
            }),
        }
    );
}

#[tokio::test]
async fn installed_agent_plugin_telemetry_metadata_uses_portable_capabilities() {
    for (skill_path, has_skills) in [
        ("skills/direct/SKILL.md", true),
        ("skills/group/nested/SKILL.md", false),
    ] {
        let codex_home = TempDir::new().unwrap();
        let plugin_root = codex_home
            .path()
            .join("plugins/cache/test/agent-plugin/local");
        write_file(
            &plugin_root.join("plugin.json"),
            r#"{"$schema":"https://agent-plugins.org/schemas/1.0.0/plugin.schema.json","name":"agent.tools"}"#,
        );
        write_file(
            &plugin_root.join(skill_path),
            "---\nname: portable\ndescription: Portable skill\n---\n",
        );
        write_file(
            &plugin_root.join("mcp.json"),
            r#"{"$schema":"https://agent-plugins.org/schemas/1.0.0/mcp.schema.json","mcpServers":{"portable":{"type":"stdio","command":"echo"}}}"#,
        );
        write_file(
            &plugin_root.join(".codex-plugin/plugin.json"),
            r#"{"apps":"./.app.json"}"#,
        );
        write_file(
            &plugin_root.join(".app.json"),
            r#"{"apps":{"legacy":{"id":"connector_legacy"}}}"#,
        );
        let manager = test_plugins_manager(codex_home.path().to_path_buf());
        let plugin_id = PluginId::parse("agent-plugin@test").expect("plugin id should parse");

        let metadata = manager
            .telemetry_metadata_for_installed_plugin(&plugin_id)
            .await;

        assert_eq!(
            metadata,
            PluginTelemetryMetadata {
                plugin_id: Some(plugin_id),
                remote_plugin_id: None,
                capability_summary: Some(PluginCapabilitySummary {
                    config_name: "agent-plugin@test".to_string(),
                    display_name: "agent-plugin".to_string(),
                    plugin_namespace: Some("agent.tools".to_string()),
                    description: None,
                    has_skills,
                    mcp_server_names: vec!["portable".to_string()],
                    app_connector_ids: Vec::new(),
                }),
            }
        );
    }
}

#[tokio::test]
async fn installed_plugin_telemetry_metadata_resolves_persisted_remote_identity() {
    let codex_home = TempDir::new().unwrap();
    write_cached_plugin(codex_home.path(), "openai-curated-remote", "linear");
    let plugin_id =
        PluginId::parse("linear@openai-curated-remote").expect("plugin id should parse");
    PluginStore::new(codex_home.path().to_path_buf())
        .write_remote_plugin_id(&plugin_id, "plugins~Plugin_linear")
        .expect("persist remote plugin id");
    let manager = test_plugins_manager(codex_home.path().to_path_buf());

    let metadata = manager
        .telemetry_metadata_for_installed_plugin(&plugin_id)
        .await;

    assert_eq!(
        metadata,
        PluginTelemetryMetadata {
            plugin_id: Some(plugin_id),
            remote_plugin_id: Some("plugins~Plugin_linear".to_string()),
            capability_summary: Some(PluginCapabilitySummary {
                config_name: "linear@openai-curated-remote".to_string(),
                display_name: "linear".to_string(),
                plugin_namespace: Some("linear".to_string()),
                description: None,
                has_skills: true,
                mcp_server_names: Vec::new(),
                app_connector_ids: Vec::new(),
            }),
        }
    );
}

#[test]
fn plugin_telemetry_ignores_local_marketplace_sidecars() {
    let codex_home = TempDir::new().unwrap();
    write_cached_plugin(codex_home.path(), "test", "sample");
    let plugin_id = PluginId::parse("sample@test").expect("plugin id should parse");
    PluginStore::new(codex_home.path().to_path_buf())
        .write_remote_plugin_id(&plugin_id, "plugins~Plugin_sample")
        .expect("persist remote plugin id");
    let manager = test_plugins_manager(codex_home.path().to_path_buf());

    assert_eq!(
        manager.telemetry_metadata_for_plugin_id(&plugin_id),
        PluginTelemetryMetadata {
            plugin_id: Some(plugin_id),
            remote_plugin_id: None,
            capability_summary: None,
        }
    );
}

#[tokio::test]
async fn installed_plugin_telemetry_metadata_prefers_remote_snapshot_identity() {
    let codex_home = TempDir::new().unwrap();
    write_cached_plugin(codex_home.path(), "openai-curated-remote", "linear");
    let plugin_id =
        PluginId::parse("linear@openai-curated-remote").expect("plugin id should parse");
    PluginStore::new(codex_home.path().to_path_buf())
        .write_remote_plugin_id(&plugin_id, "plugins~Plugin_stale")
        .expect("persist remote plugin id");
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    manager.write_remote_installed_plugins_cache(vec![remote_installed_linear_plugin()]);

    let metadata = manager
        .telemetry_metadata_for_installed_plugin(&plugin_id)
        .await;

    assert_eq!(
        metadata,
        PluginTelemetryMetadata {
            plugin_id: Some(plugin_id),
            remote_plugin_id: Some("plugins~Plugin_linear".to_string()),
            capability_summary: Some(PluginCapabilitySummary {
                config_name: "linear@openai-curated-remote".to_string(),
                display_name: "linear".to_string(),
                plugin_namespace: Some("linear".to_string()),
                description: None,
                has_skills: true,
                mcp_server_names: Vec::new(),
                app_connector_ids: Vec::new(),
            }),
        }
    );
}

#[tokio::test]
async fn installed_plugin_telemetry_metadata_accepts_authoritative_remote_identity() {
    let codex_home = TempDir::new().unwrap();
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let plugin_id =
        PluginId::parse("linear@openai-curated-remote").expect("plugin id should parse");

    let metadata = manager
        .telemetry_metadata_for_installed_plugin_with_remote_id(&plugin_id, "plugins~Plugin_linear")
        .await;

    assert_eq!(
        metadata,
        PluginTelemetryMetadata {
            plugin_id: Some(plugin_id),
            remote_plugin_id: Some("plugins~Plugin_linear".to_string()),
            capability_summary: None,
        }
    );
}

#[test]
fn capability_summary_telemetry_metadata_uses_local_identity() {
    let codex_home = TempDir::new().unwrap();
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let summary = PluginCapabilitySummary {
        config_name: "linear@openai-curated-remote".to_string(),
        display_name: "Linear".to_string(),
        plugin_namespace: Some("linear".to_string()),
        description: Some("Track work".to_string()),
        has_skills: true,
        mcp_server_names: vec!["linear".to_string()],
        app_connector_ids: vec![AppConnectorId("linear-app".to_string())],
    };

    let metadata = manager.telemetry_metadata_for_capability_summary(&summary);

    assert_eq!(
        metadata,
        Some(PluginTelemetryMetadata {
            plugin_id: Some(
                PluginId::parse("linear@openai-curated-remote").expect("plugin id should parse"),
            ),
            remote_plugin_id: None,
            capability_summary: Some(summary),
        })
    );
}

#[test]
fn capability_summary_telemetry_metadata_resolves_persisted_remote_identity() {
    let codex_home = TempDir::new().unwrap();
    write_cached_plugin(codex_home.path(), "openai-curated-remote", "linear");
    let plugin_id =
        PluginId::parse("linear@openai-curated-remote").expect("plugin id should parse");
    PluginStore::new(codex_home.path().to_path_buf())
        .write_remote_plugin_id(&plugin_id, "plugins~Plugin_linear")
        .expect("persist remote plugin id");
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let summary = PluginCapabilitySummary {
        config_name: "linear@openai-curated-remote".to_string(),
        display_name: "Linear".to_string(),
        plugin_namespace: Some("linear".to_string()),
        description: Some("Track work".to_string()),
        has_skills: true,
        mcp_server_names: vec!["linear".to_string()],
        app_connector_ids: vec![AppConnectorId("linear-app".to_string())],
    };

    let metadata = manager.telemetry_metadata_for_capability_summary(&summary);

    assert_eq!(
        metadata,
        Some(PluginTelemetryMetadata {
            plugin_id: Some(plugin_id),
            remote_plugin_id: Some("plugins~Plugin_linear".to_string()),
            capability_summary: Some(summary),
        })
    );
}





#[tokio::test]
async fn build_remote_installed_plugin_marketplaces_from_cache_uses_remote_metadata() {
    let codex_home = TempDir::new().unwrap();
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let mut plugin = remote_installed_linear_plugin();
    plugin.install_policy = codex_app_server_protocol::PluginInstallPolicy::InstalledByDefault;
    plugin.auth_policy = codex_app_server_protocol::PluginAuthPolicy::OnInstall;
    plugin.interface = Some(codex_app_server_protocol::PluginInterface {
        display_name: Some("Linear".to_string()),
        short_description: Some("Track remote work".to_string()),
        long_description: None,
        developer_name: None,
        category: None,
        capabilities: Vec::new(),
        website_url: None,
        privacy_policy_url: None,
        terms_of_service_url: None,
        default_prompt: None,
        brand_color: Some("#111111".to_string()),
        composer_icon: None,
        composer_icon_url: None,
        logo: None,
        logo_dark: None,
        logo_url: None,
        logo_url_dark: None,
        screenshots: Vec::new(),
        screenshot_urls: Vec::new(),
    });
    plugin.keywords = vec!["issues".to_string()];
    manager.write_remote_installed_plugins_cache(vec![plugin]);

    let marketplaces = manager
        .build_remote_installed_plugin_marketplaces_from_cache(&[REMOTE_GLOBAL_MARKETPLACE_NAME])
        .expect("remote installed cache should be present");
    assert_eq!(marketplaces.len(), 1);
    assert_eq!(marketplaces[0].name, "openai-curated-remote");
    assert_eq!(marketplaces[0].display_name, "OpenAI Curated Remote");
    assert_eq!(marketplaces[0].plugins.len(), 1);
    let plugin = &marketplaces[0].plugins[0];
    assert_eq!(plugin.id, "linear@openai-curated-remote");
    assert_eq!(plugin.remote_plugin_id, "plugins~Plugin_linear");
    assert_eq!(plugin.name, "linear");
    assert_eq!(plugin.installed, true);
    assert_eq!(plugin.enabled, true);
    assert_eq!(
        plugin.install_policy,
        codex_app_server_protocol::PluginInstallPolicy::InstalledByDefault
    );
    assert_eq!(
        plugin.auth_policy,
        codex_app_server_protocol::PluginAuthPolicy::OnInstall
    );
    assert_eq!(plugin.keywords, vec!["issues".to_string()]);
    assert_eq!(
        plugin
            .interface
            .as_ref()
            .and_then(|interface| interface.display_name.as_deref()),
        Some("Linear")
    );
    assert_eq!(
        plugin
            .interface
            .as_ref()
            .and_then(|interface| interface.short_description.as_deref()),
        Some("Track remote work")
    );
    assert_eq!(
        manager
            .build_remote_installed_plugin_marketplaces_from_cache(&[
                REMOTE_WORKSPACE_MARKETPLACE_NAME
            ])
            .expect("remote installed cache should be present"),
        Vec::new()
    );
}

#[tokio::test]
async fn build_remote_installed_plugin_marketplaces_from_cache_filters_by_marketplace_name() {
    let codex_home = TempDir::new().unwrap();
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    manager.write_remote_installed_plugins_cache(vec![
        remote_installed_plugin_in_marketplace(
            "workspace-linear",
            REMOTE_WORKSPACE_MARKETPLACE_NAME,
        ),
        remote_installed_plugin_in_marketplace(
            "shared-linear",
            REMOTE_WORKSPACE_SHARED_WITH_ME_MARKETPLACE_NAME,
        ),
    ]);

    let marketplaces = manager
        .build_remote_installed_plugin_marketplaces_from_cache(&[REMOTE_WORKSPACE_MARKETPLACE_NAME])
        .expect("remote installed cache should be present");

    assert_eq!(marketplaces.len(), 1);
    assert_eq!(marketplaces[0].name, REMOTE_WORKSPACE_MARKETPLACE_NAME);
    assert_eq!(
        marketplaces[0]
            .plugins
            .iter()
            .map(|plugin| plugin.id.as_str())
            .collect::<Vec<_>>(),
        vec!["workspace-linear@workspace-directory"]
    );
}

#[tokio::test]
async fn load_plugins_resolves_disabled_skill_names_against_loaded_plugin_skills() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");
    let skill_path = plugin_root.join("skills/sample-search/SKILL.md");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    );
    write_file(
        &skill_path,
        "---\nname: sample-search\ndescription: search sample data\n---\n",
    );

    let config_toml = r#"[features]
plugins = true

[[skills.config]]
name = "sample:sample-search"
enabled = false

[plugins."sample@test"]
enabled = true
"#;
    let outcome =
        load_plugins_from_config(config_toml, codex_home.path(), /*auth_mode*/ None).await;
    let skill_path = std::fs::canonicalize(skill_path)
        .expect("skill path should canonicalize")
        .abs();

    assert_eq!(
        outcome.plugins()[0].disabled_skill_paths,
        HashSet::from([skill_path])
    );
    assert!(!outcome.plugins()[0].has_enabled_skills);
    assert!(outcome.capability_summaries().is_empty());
}

#[tokio::test]
async fn load_plugins_ignores_unknown_disabled_skill_names() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    );
    write_file(
        &plugin_root.join("skills/sample-search/SKILL.md"),
        "---\nname: sample-search\ndescription: search sample data\n---\n",
    );

    let config_toml = r#"[features]
plugins = true

[[skills.config]]
name = "sample:missing-skill"
enabled = false

[plugins."sample@test"]
enabled = true
"#;
    let outcome =
        load_plugins_from_config(config_toml, codex_home.path(), /*auth_mode*/ None).await;

    assert!(outcome.plugins()[0].disabled_skill_paths.is_empty());
    assert!(outcome.plugins()[0].has_enabled_skills);
    assert_eq!(
        outcome.capability_summaries(),
        &[PluginCapabilitySummary {
            config_name: "sample@test".to_string(),
            display_name: "sample".to_string(),
            plugin_namespace: Some("sample".to_string()),
            description: None,
            has_skills: true,
            mcp_server_names: Vec::new(),
            app_connector_ids: Vec::new(),
        }]
    );
}

#[tokio::test]
async fn plugin_telemetry_metadata_uses_default_mcp_config_path() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{
  "name": "sample"
}"#,
    );
    write_file(
        &plugin_root.join(".mcp.json"),
        r#"{
  "mcpServers": {
    "sample": {
      "type": "http",
      "url": "https://sample.example/mcp"
    }
  }
}"#,
    );

    let summary = plugin_capability_summary_from_root(
        &PluginId::parse("sample@test").expect("plugin id should parse"),
        &plugin_root.abs(),
        test_skill_root_loader().as_ref(),
    )
    .await;

    assert_eq!(
        summary,
        Some(PluginCapabilitySummary {
            config_name: "sample@test".to_string(),
            display_name: "sample".to_string(),
            plugin_namespace: Some("sample".to_string()),
            description: None,
            has_skills: false,
            mcp_server_names: vec!["sample".to_string()],
            app_connector_ids: Vec::new(),
        })
    );
}

#[tokio::test]
async fn plugin_capability_summary_uses_manifest_mcp_server_objects() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/counter-sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{
  "name": "counter-sample",
  "version": "1.1.1",
  "mcpServers": {
    "counter": {
      "type": "http",
      "url": "https://sample.example/counter/mcp"
    }
  }
}"#,
    );

    let summary = plugin_capability_summary_from_root(
        &PluginId::parse("counter-sample@test").expect("plugin id should parse"),
        &plugin_root.abs(),
        test_skill_root_loader().as_ref(),
    )
    .await;

    assert_eq!(
        summary,
        Some(PluginCapabilitySummary {
            config_name: "counter-sample@test".to_string(),
            display_name: "counter-sample".to_string(),
            plugin_namespace: Some("counter-sample".to_string()),
            description: None,
            has_skills: false,
            mcp_server_names: vec!["counter".to_string()],
            app_connector_ids: Vec::new(),
        })
    );
}

#[tokio::test]
async fn capability_summary_sanitizes_plugin_descriptions_to_one_line() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{
  "name": "sample",
  "description": "Plugin that\n includes   the sample\tserver"
}"#,
    );
    write_file(
        &plugin_root.join("skills/sample-search/SKILL.md"),
        "---\nname: sample-search\ndescription: search sample data\n---\n",
    );

    let outcome = load_plugins_from_config(
        &plugin_config_toml(/*enabled*/ true, /*plugins_feature_enabled*/ true),
        codex_home.path(),
        /*auth_mode*/ None,
    )
    .await;

    assert_eq!(
        outcome.plugins()[0].manifest_description.as_deref(),
        Some("Plugin that\n includes   the sample\tserver")
    );
    assert_eq!(
        outcome.capability_summaries()[0].description.as_deref(),
        Some("Plugin that includes the sample server")
    );
}

#[tokio::test]
async fn capability_summary_truncates_overlong_plugin_descriptions() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");
    let too_long = "x".repeat(MAX_CAPABILITY_SUMMARY_DESCRIPTION_LEN + 1);

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        &format!(
            r#"{{
  "name": "sample",
  "description": "{too_long}"
}}"#
        ),
    );
    write_file(
        &plugin_root.join("skills/sample-search/SKILL.md"),
        "---\nname: sample-search\ndescription: search sample data\n---\n",
    );

    let outcome = load_plugins_from_config(
        &plugin_config_toml(/*enabled*/ true, /*plugins_feature_enabled*/ true),
        codex_home.path(),
        /*auth_mode*/ None,
    )
    .await;

    assert_eq!(
        outcome.plugins()[0].manifest_description.as_deref(),
        Some(too_long.as_str())
    );
    assert_eq!(
        outcome.capability_summaries()[0].description,
        Some("x".repeat(MAX_CAPABILITY_SUMMARY_DESCRIPTION_LEN))
    );
}


#[tokio::test]
async fn install_plugin_materializes_default_command_skills() {
    let codex_home = TempDir::new().unwrap();
    let source_root = codex_home.path().join("source/sample");

    write_file(
        &source_root.join(".codex-plugin/plugin.json"),
        r#"{
  "name": "sample",
  "skills": "./custom-skills/"
}"#,
    );
    fs::create_dir_all(source_root.join("custom-skills")).unwrap();
    write_file(
        &source_root.join("custom-skills/source-command-pr-review/SKILL.md"),
        "---\nname: source-command-pr-review\ndescription: Native review skill\n---\n",
    );
    write_file(
        &source_root.join("commands/pr/review.md"),
        "---\ndescription: Review a pull request\n---\nInspect the proposed changes.\n",
    );
    write_file(
        &source_root.join("commands/summarize.md"),
        "---\ndescription: Summarize a change\n---\nSummarize the proposed changes.\n",
    );
    write_file(
        &source_root.join("commands/oversized.md"),
        &format!("---\ndescription: Oversized\n---\n{}", "x".repeat(4_000)),
    );
    write_file(
        &source_root.join(".codex-plugin/migrated-command-skills/undeclared-command/SKILL.md"),
        "---\nname: undeclared-command\ndescription: undeclared command\n---\n",
    );
    let result = PluginStore::new(codex_home.path().to_path_buf())
        .install(
            source_root.abs(),
            PluginId::parse("sample@test").expect("plugin id should parse"),
        )
        .unwrap();
    let migrated_skill = result
        .installed_path
        .join(".codex-plugin/migrated-command-skills/source-command-pr-review/SKILL.md");
    let expected_migrated_skill = "---\nname: \"source-command-pr-review\"\ndescription: \"Review a pull request\"\n---\n\n# source-command-pr-review\n\nUse this skill when the user asks to run the migrated source command `pr-review`.\n\n## Command Template\n\nInspect the proposed changes.\n";
    assert_eq!(
        fs::read_to_string(&migrated_skill).unwrap(),
        expected_migrated_skill
    );
    assert!(
        !result
            .installed_path
            .join(".codex-plugin/migrated-command-skills/undeclared-command")
            .exists()
    );
    assert!(
        !result
            .installed_path
            .join(".codex-plugin/migrated-command-skills/source-command-oversized")
            .exists()
    );

    let manifest = crate::manifest::load_plugin_manifest(&result.installed_path).unwrap();
    let resolved = load_plugin_skill_inventory(
        &result.installed_path,
        &PluginIdentity {
            plugin_id: result.plugin_id.as_key(),
            remote_plugin_id: None,
        },
        &manifest,
        PluginManifestFormat::Legacy,
        /*restriction_product*/ None,
        /*plugin_skill_snapshots*/ None,
        test_skill_root_loader().as_ref(),
    )
    .await
    .resolve(&SkillConfigRules::default());
    assert_eq!(
        resolved
            .skills
            .iter()
            .map(|skill| skill.path_to_skills_md.clone())
            .collect::<Vec<_>>(),
        vec![
            AbsolutePathBuf::from_absolute_path_checked(
                fs::canonicalize(
                    result
                        .installed_path
                        .join("custom-skills/source-command-pr-review/SKILL.md")
                )
                .unwrap()
            )
            .unwrap(),
            AbsolutePathBuf::from_absolute_path_checked(
                fs::canonicalize(result.installed_path.join(
                    ".codex-plugin/migrated-command-skills/source-command-summarize/SKILL.md"
                ))
                .unwrap()
            )
            .unwrap()
        ]
    );
}

#[test]
fn install_plugin_ignores_invalid_commands_manifest_field() {
    let codex_home = TempDir::new().unwrap();
    let source_root = codex_home.path().join("source/sample");
    write_file(
        &source_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample","commands":{}}"#,
    );
    write_file(
        &source_root.join("commands/review.md"),
        "---\ndescription: Review\n---\nReview the current change.\n",
    );

    let result = PluginStore::new(codex_home.path().to_path_buf())
        .install(
            source_root.abs(),
            PluginId::parse("sample@test").expect("plugin id should parse"),
        )
        .unwrap();

    assert!(
        !result
            .installed_path
            .join(".codex-plugin/migrated-command-skills")
            .exists()
    );
}

#[test]
fn install_plugin_ignores_command_migration_errors() {
    let codex_home = TempDir::new().unwrap();
    let source_root = codex_home.path().join("source/sample");
    write_file(
        &source_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample","commands":"./commands/review.md"}"#,
    );
    fs::create_dir_all(source_root.join("commands")).unwrap();
    fs::write(source_root.join("commands/review.md"), [0xff]).unwrap();

    let result = PluginStore::new(codex_home.path().to_path_buf())
        .install(
            source_root.abs(),
            PluginId::parse("sample@test").expect("plugin id should parse"),
        )
        .unwrap();

    assert!(result.installed_path.join("commands/review.md").is_file());
}

#[tokio::test]
async fn load_plugin_skills_dedupes_overlapping_manifest_roots() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local")
        .abs();
    write_file(
        &plugin_root.join("skills/abc/SKILL.md"),
        "---\nname: abc\ndescription: abc skill\n---\n",
    );
    write_file(
        &plugin_root.join("skills/edk/SKILL.md"),
        "---\nname: edk\ndescription: edk skill\n---\n",
    );
    let manifest = crate::manifest::PluginManifest {
        name: "sample".to_string(),
        version: None,
        description: None,
        keywords: Vec::new(),
        paths: crate::manifest::PluginManifestPaths {
            skills: vec![
                plugin_root.join("skills"),
                plugin_root.join("skills/abc"),
                plugin_root.join("skills/edk"),
                plugin_root.join("skills/abc"),
            ],
            mcp_servers: None,
            apps: None,
            hooks: None,
        },
        interface: None,
    };
    let plugin_id = PluginId::parse("sample@test").expect("plugin id should parse");
    let resolved = load_plugin_skill_inventory(
        &plugin_root,
        &PluginIdentity {
            plugin_id: plugin_id.as_key(),
            remote_plugin_id: None,
        },
        &manifest,
        PluginManifestFormat::Legacy,
        /*restriction_product*/ None,
        /*plugin_skill_snapshots*/ None,
        test_skill_root_loader().as_ref(),
    )
    .await
    .resolve(&SkillConfigRules::default());

    let skill_paths = resolved
        .skills
        .iter()
        .map(|skill| skill.path_to_skills_md.clone())
        .collect::<Vec<_>>();
    let canonical_skill_path = |path| {
        AbsolutePathBuf::from_absolute_path_checked(
            fs::canonicalize(plugin_root.join(path)).expect("canonical skill path"),
        )
        .expect("absolute skill path")
    };
    assert_eq!(
        skill_paths,
        vec![
            canonical_skill_path("skills/abc/SKILL.md"),
            canonical_skill_path("skills/edk/SKILL.md")
        ]
    );
}


#[tokio::test]
async fn load_plugins_ignores_invalid_manifest_skills_shape() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{
  "name": "sample",
  "skills": { "path": "./custom-skills/" }
}"#,
    );
    write_file(
        &plugin_root.join("skills/default-skill/SKILL.md"),
        "---\nname: default-skill\ndescription: default skill\n---\n",
    );
    write_file(
        &plugin_root.join("custom-skills/custom-skill/SKILL.md"),
        "---\nname: custom-skill\ndescription: custom skill\n---\n",
    );

    let outcome = load_plugins_from_config(
        &plugin_config_toml(/*enabled*/ true, /*plugins_feature_enabled*/ true),
        codex_home.path(),
        /*auth_mode*/ None,
    )
    .await;

    assert_eq!(outcome.plugins()[0].error, None);
    assert_eq!(
        outcome.plugins()[0].skill_roots,
        vec![plugin_root.join("skills").abs()]
    );
}

#[tokio::test]
async fn load_plugins_preserves_disabled_plugins_without_effective_contributions() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    );
    write_file(
        &plugin_root.join(".mcp.json"),
        r#"{
  "mcpServers": {
    "sample": {
      "type": "http",
      "url": "https://sample.example/mcp"
    }
  }
}"#,
    );

    let outcome = load_plugins_from_config(
        &plugin_config_toml(
            /*enabled*/ false, /*plugins_feature_enabled*/ true,
        ),
        codex_home.path(),
        /*auth_mode*/ None,
    )
    .await;

    assert_eq!(
        outcome.plugins(),
        vec![LoadedPlugin {
            config_name: "sample@test".to_string(),
            remote_plugin_id: None,
            manifest_name: None,
            plugin_namespace: None,
            manifest_description: None,
            root: AbsolutePathBuf::try_from(plugin_root).unwrap(),
            enabled: false,
            skill_roots: Vec::new(),
            skill_discovery_mode: SkillDiscoveryMode::Recursive,
            disabled_skill_paths: HashSet::new(),
            has_enabled_skills: false,
            mcp_servers: HashMap::new(),
            apps: Vec::new(),
            hook_sources: Vec::new(),
            hook_load_warnings: Vec::new(),
            error: None,
        }]
    );
    assert!(outcome.effective_plugin_skill_roots().is_empty());
    assert!(outcome.effective_mcp_servers().is_empty());
}



#[test]
fn capability_index_filters_inactive_and_zero_capability_plugins() {
    let codex_home = TempDir::new().unwrap();
    let connector = |id: &str| AppConnectorId(id.to_string());
    let app = |name: &str, connector_id: &str| app_declaration(name, connector_id);
    let http_server = |url: &str| McpServerConfig {
        auth: Default::default(),
        transport: McpServerTransportConfig::StreamableHttp {
            url: url.to_string(),
            bearer_token_env_var: None,
            http_headers: None,
            env_http_headers: None,
            http_headers_helper: None,
        },
        environment_id: "local".to_string(),
        enabled: true,
        required: false,
        supports_parallel_tool_calls: false,
        omit_tools_from: None,
        disabled_reason: None,
        startup_timeout_sec: None,
        tool_timeout_sec: None,
        default_tools_approval_mode: None,
        enabled_tools: None,
        disabled_tools: None,
        scopes: None,
        oauth: None,
        oauth_resource: None,
        tools: HashMap::new(),
    };
    let plugin = |config_name: &str, dir_name: &str, manifest_name: &str| LoadedPlugin {
        config_name: config_name.to_string(),
        remote_plugin_id: None,
        manifest_name: Some(manifest_name.to_string()),
        plugin_namespace: Some(
            config_name
                .split_once('@')
                .map_or(config_name, |(name, _)| name)
                .to_string(),
        ),
        manifest_description: None,
        root: AbsolutePathBuf::try_from(codex_home.path().join(dir_name)).unwrap(),
        enabled: true,
        skill_roots: Vec::new(),
        skill_discovery_mode: SkillDiscoveryMode::Recursive,
        disabled_skill_paths: HashSet::new(),
        has_enabled_skills: false,
        mcp_servers: HashMap::new(),
        apps: Vec::new(),
        hook_sources: Vec::new(),
        hook_load_warnings: Vec::new(),
        error: None,
    };
    let summary = |config_name: &str, display_name: &str| PluginCapabilitySummary {
        config_name: config_name.to_string(),
        display_name: display_name.to_string(),
        plugin_namespace: Some(
            config_name
                .split_once('@')
                .map_or(config_name, |(name, _)| name)
                .to_string(),
        ),
        description: None,
        ..PluginCapabilitySummary::default()
    };
    let outcome = PluginLoadOutcome::from_plugins(vec![
        LoadedPlugin {
            skill_roots: vec![codex_home.path().join("skills-plugin/skills").abs()],
            has_enabled_skills: true,
            ..plugin("skills@test", "skills-plugin", "skills-plugin")
        },
        LoadedPlugin {
            mcp_servers: HashMap::from([("alpha".to_string(), http_server("https://alpha"))]),
            apps: vec![app("example", "connector_example")],
            ..plugin("alpha@test", "alpha-plugin", "alpha-plugin")
        },
        LoadedPlugin {
            mcp_servers: HashMap::from([("beta".to_string(), http_server("https://beta"))]),
            apps: vec![
                app("example", "connector_example"),
                app("gmail", "connector_gmail"),
            ],
            ..plugin("beta@test", "beta-plugin", "beta-plugin")
        },
        plugin("empty@test", "empty-plugin", "empty-plugin"),
        LoadedPlugin {
            enabled: false,
            skill_roots: vec![codex_home.path().join("disabled-plugin/skills").abs()],
            apps: vec![app("hidden", "connector_hidden")],
            ..plugin("disabled@test", "disabled-plugin", "disabled-plugin")
        },
        LoadedPlugin {
            apps: vec![app("broken", "connector_broken")],
            error: Some("failed to load".to_string()),
            ..plugin("broken@test", "broken-plugin", "broken-plugin")
        },
    ]);

    assert_eq!(
        outcome.capability_summaries(),
        &[
            PluginCapabilitySummary {
                has_skills: true,
                ..summary("skills@test", "skills-plugin")
            },
            PluginCapabilitySummary {
                mcp_server_names: vec!["alpha".to_string()],
                app_connector_ids: vec![connector("connector_example")],
                ..summary("alpha@test", "alpha-plugin")
            },
            PluginCapabilitySummary {
                mcp_server_names: vec!["beta".to_string()],
                app_connector_ids: vec![
                    connector("connector_example"),
                    connector("connector_gmail"),
                ],
                ..summary("beta@test", "beta-plugin")
            },
        ]
    );
}

#[tokio::test]
async fn load_plugins_returns_empty_when_feature_disabled() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    );
    write_file(
        &plugin_root.join("skills/sample-search/SKILL.md"),
        "---\nname: sample-search\ndescription: search sample data\n---\n",
    );
    write_file(
        &codex_home.path().join(CONFIG_TOML_FILE),
        &plugin_config_toml(
            /*enabled*/ true, /*plugins_feature_enabled*/ false,
        ),
    );

    let config = load_config(codex_home.path(), codex_home.path()).await;
    let outcome = test_plugins_manager(codex_home.path().to_path_buf())
        .plugins_for_config(&config)
        .await;

    assert_eq!(outcome, PluginLoadOutcome::default());
}

#[tokio::test]
async fn plugin_cache_reuses_effective_configurations() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");
    write_plugin(
        codex_home.path().join("plugins/cache/test").as_path(),
        "sample/local",
        "sample",
    );
    write_file(
        &plugin_root.join(".mcp.json"),
        r#"{
  "mcpServers": {
    "sample": {
      "url": "https://sample.example/mcp"
    }
  }
}"#,
    );

    let user_file = codex_home.path().join(CONFIG_TOML_FILE).abs();
    let user_config: toml::Value = toml::from_str(&plugin_config_toml(
        /*enabled*/ true, /*plugins_feature_enabled*/ true,
    ))
    .expect("user config should parse");
    let stack = |session_config: &str| {
        ConfigLayerStack::new(
            vec![
                ConfigLayerEntry::new(
                    ConfigLayerSource::User {
                        file: user_file.clone(),
                        profile: None,
                    },
                    user_config.clone(),
                ),
                ConfigLayerEntry::new(
                    ConfigLayerSource::SessionFlags,
                    toml::from_str(session_config).expect("session config should parse"),
                ),
            ],
            ConfigRequirements::default(),
            ConfigRequirementsToml::default(),
        )
        .expect("config layer stack should build")
    };
    let config = |session_config| {
        PluginsConfigInput::new(
            stack(session_config),
            String::new(),
            /*plugins_enabled*/ true,
            /*remote_plugin_enabled*/ false,
            "https://chatgpt.com".to_string(),
            test_http_client_factory(),
        )
    };
    let manager = test_plugins_manager(codex_home.path().to_path_buf());

    let first_config = config(r#"model = "first""#);
    let second_config = config(
        r#"[plugins."sample@test".mcp_servers.sample]
enabled = false"#,
    );
    let first = manager.plugins_for_config(&first_config).await;
    let first_snapshots = manager
        .plugin_skill_snapshots_for_config(&first_config)
        .expect("first configuration snapshots");
    let second = manager.plugins_for_config(&second_config).await;
    let second_snapshots = manager
        .plugin_skill_snapshots_for_config(&second_config)
        .expect("second configuration snapshots");
    std::fs::remove_file(plugin_root.join(".mcp.json")).unwrap();

    assert_eq!(
        manager.plugin_skill_snapshots_for_config(&first_config),
        Some(first_snapshots),
    );
    assert_eq!(
        manager
            .plugins_for_config(&config(r#"model = "second""#))
            .await,
        first,
    );
    assert_eq!(
        manager.plugin_skill_snapshots_for_config(&second_config),
        Some(second_snapshots),
    );
    assert_eq!(manager.plugins_for_config(&second_config).await, second);
    manager.clear_cache();
    assert_eq!(
        [first_config, second_config]
            .iter()
            .map(|config| manager.plugin_skill_snapshots_for_config(config))
            .collect::<Vec<_>>(),
        vec![None, None],
    );
}


#[test]
fn loaded_plugins_cache_evicts_least_recently_used_configuration() {
    let codex_home = TempDir::new().unwrap();
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let keys = (0..=LOADED_PLUGINS_CACHE_CAPACITY)
        .map(|index| PluginLoadCacheKey {
            configured_plugins: HashMap::from([(
                format!("plugin-{index}@test"),
                PluginConfig {
                    enabled: true,
                    mcp_servers: HashMap::new(),
                },
            )]),
            skill_config_rules: SkillConfigRules::default(),
            remote_global_catalog_active: false,
            auth_identity: None,
        })
        .collect::<Vec<_>>();
    let generation = manager.loaded_plugins_cache_generation();
    for key in keys.iter().take(LOADED_PLUGINS_CACHE_CAPACITY) {
        manager.cache_loaded_plugins_if_current(
            generation,
            key.clone(),
            Vec::new(),
            crate::skill_snapshots::new_plugin_skill_snapshots(),
        );
    }
    manager.cache_loaded_plugins_if_current(
        generation,
        keys[LOADED_PLUGINS_CACHE_CAPACITY - 1].clone(),
        Vec::new(),
        crate::skill_snapshots::new_plugin_skill_snapshots(),
    );
    assert_eq!(manager.cached_loaded_plugins(&keys[0]), Some(Vec::new()));
    manager.cache_loaded_plugins_if_current(
        generation,
        keys[LOADED_PLUGINS_CACHE_CAPACITY].clone(),
        Vec::new(),
        crate::skill_snapshots::new_plugin_skill_snapshots(),
    );

    assert_eq!(
        keys.iter()
            .map(|key| manager.cached_loaded_plugins(key).is_some())
            .collect::<Vec<_>>(),
        (0..=LOADED_PLUGINS_CACHE_CAPACITY)
            .map(|index| index != 1)
            .collect::<Vec<_>>(),
    );
}

#[test]
fn loaded_plugins_cache_invalidation_rejects_stale_load_completion() {
    let codex_home = TempDir::new().unwrap();
    let manager = test_plugins_manager(codex_home.path().to_path_buf());
    let cache_key = PluginLoadCacheKey {
        configured_plugins: HashMap::new(),
        skill_config_rules: SkillConfigRules::default(),
        remote_global_catalog_active: false,
        auth_identity: None,
    };
    let stale_generation = manager.loaded_plugins_cache_generation();

    manager.clear_loaded_plugins_cache();
    manager.cache_loaded_plugins_if_current(
        stale_generation,
        cache_key.clone(),
        Vec::new(),
        crate::skill_snapshots::new_plugin_skill_snapshots(),
    );

    assert_eq!(manager.cached_loaded_plugins(&cache_key), None);
}


#[tokio::test]
async fn load_plugins_rejects_invalid_plugin_keys() {
    let codex_home = TempDir::new().unwrap();
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");

    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    );

    let mut root = toml::map::Map::new();
    let mut features = toml::map::Map::new();
    features.insert("plugins".to_string(), Value::Boolean(true));
    root.insert("features".to_string(), Value::Table(features));

    let mut plugin = toml::map::Map::new();
    plugin.insert("enabled".to_string(), Value::Boolean(true));

    let mut plugins = toml::map::Map::new();
    plugins.insert("sample".to_string(), Value::Table(plugin));
    root.insert("plugins".to_string(), Value::Table(plugins));

    let outcome = load_plugins_from_config(
        &toml::to_string(&Value::Table(root)).expect("plugin test config should serialize"),
        codex_home.path(),
        /*auth_mode*/ None,
    )
    .await;

    assert_eq!(outcome.plugins().len(), 1);
    assert_eq!(
        outcome.plugins()[0].error.as_deref(),
        Some("invalid plugin key `sample`; expected <plugin>@<marketplace>")
    );
    assert!(outcome.effective_plugin_skill_roots().is_empty());
    assert!(outcome.effective_mcp_servers().is_empty());
}

#[tokio::test]
async fn install_plugin_updates_config_with_relative_path_and_plugin_key() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    write_plugin(&repo_root, "sample-plugin", "sample-plugin");
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "sample-plugin",
      "source": {
        "source": "local",
        "path": "./sample-plugin"
      },
      "policy": {
        "authentication": "ON_USE"
      }
    }
  ]
}"#,
    )
    .unwrap();

    let result = test_plugins_manager(tmp.path().to_path_buf())
        .install_plugin(
            &unrestricted_config_layer_stack(),
            PluginInstallRequest {
                plugin_name: "sample-plugin".to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    repo_root.join(".agents/plugins/marketplace.json"),
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();

    let installed_path = tmp.path().join("plugins/cache/debug/sample-plugin/local");
    assert_eq!(
        result,
        PluginInstallOutcome {
            plugin_id: PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap(),
            plugin_version: "local".to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path).unwrap(),
            auth_policy: MarketplacePluginAuthPolicy::OnUse,
        }
    );

    let config = fs::read_to_string(tmp.path().join("config.toml")).unwrap();
    assert!(config.contains(r#"[plugins."sample-plugin@debug"]"#));
    assert!(config.contains("enabled = true"));
}

#[tokio::test]
async fn strict_install_requires_allowed_local_marketplace_to_be_added_first() {
    let codex_home = TempDir::new().expect("create Codex home");
    let marketplace_root = codex_home.path().join("company-marketplace");
    write_plugin(&marketplace_root, "sample", "sample");
    write_file(
        &marketplace_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "company",
  "plugins": [
    {
      "name": "sample",
      "source": {"source": "local", "path": "./sample"}
    }
  ]
}"#,
    );
    let marketplace_root = marketplace_root
        .canonicalize()
        .expect("canonical marketplace root");
    let requirements = format!(
        r#"
[marketplaces]
restrict_to_allowed_sources = true

[marketplaces.allowed_sources.company]
source = "local"
path = {marketplace_root:?}
"#
    );
    let config = config_layer_stack_with_requirements(codex_home.path(), "", &requirements);
    let marketplace_path =
        AbsolutePathBuf::try_from(marketplace_root.join(".agents/plugins/marketplace.json"))
            .expect("absolute marketplace path");
    let manager = test_plugins_manager(codex_home.path().to_path_buf());

    let err = manager
        .install_plugin(
            &config,
            PluginInstallRequest {
                plugin_name: "sample".to_string(),
                marketplace_path: marketplace_path.clone(),
            },
        )
        .await
        .expect_err("unconfigured local marketplace should not be installable in strict mode");
    assert!(matches!(
        err,
        PluginInstallError::Marketplace(MarketplaceError::InvalidMarketplaceFile { .. })
    ));
    assert!(err.to_string().contains("must be added to config"));
    assert!(!codex_home.path().join(CONFIG_TOML_FILE).exists());

    let user_config = format!(
        r#"
[marketplaces.company]
source_type = "local"
source = {marketplace_root:?}
"#
    );
    write_file(&codex_home.path().join(CONFIG_TOML_FILE), &user_config);
    let config =
        config_layer_stack_with_requirements(codex_home.path(), &user_config, &requirements);
    let outcome = manager
        .install_plugin(
            &config,
            PluginInstallRequest {
                plugin_name: "sample".to_string(),
                marketplace_path,
            },
        )
        .await
        .expect("configured allowlisted marketplace should be installable");
    assert_eq!(
        outcome.plugin_id,
        PluginId::new("sample".to_string(), "company".to_string()).expect("plugin id")
    );
}

#[tokio::test]
async fn install_openai_curated_plugin_uses_short_sha_cache_version() {
    let tmp = tempfile::tempdir().unwrap();
    let curated_root = curated_plugins_repo_path(tmp.path());
    write_openai_curated_marketplace(&curated_root, &["slack"]);
    write_curated_plugin_sha(tmp.path(), TEST_CURATED_PLUGIN_SHA);

    let result = test_plugins_manager(tmp.path().to_path_buf())
        .install_plugin(
            &unrestricted_config_layer_stack(),
            PluginInstallRequest {
                plugin_name: "slack".to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    curated_root.join(".agents/plugins/marketplace.json"),
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();

    let installed_path = tmp.path().join(format!(
        "plugins/cache/openai-curated/slack/{TEST_CURATED_PLUGIN_CACHE_VERSION}"
    ));
    assert_eq!(
        result,
        PluginInstallOutcome {
            plugin_id: PluginId::new(
                "slack".to_string(),
                OPENAI_CURATED_MARKETPLACE_NAME.to_string()
            )
            .unwrap(),
            plugin_version: TEST_CURATED_PLUGIN_CACHE_VERSION.to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path).unwrap(),
            auth_policy: MarketplacePluginAuthPolicy::OnInstall,
        }
    );
}

#[tokio::test]
async fn install_plugin_uses_manifest_version_for_non_curated_plugins() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    write_plugin_with_version(
        &repo_root,
        "sample-plugin",
        "sample-plugin",
        Some("1.2.3-beta+7"),
    );
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "sample-plugin",
      "source": {
        "source": "local",
        "path": "./sample-plugin"
      }
    }
  ]
}"#,
    )
    .unwrap();

    let result = test_plugins_manager(tmp.path().to_path_buf())
        .install_plugin(
            &unrestricted_config_layer_stack(),
            PluginInstallRequest {
                plugin_name: "sample-plugin".to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    repo_root.join(".agents/plugins/marketplace.json"),
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();

    let installed_path = tmp
        .path()
        .join("plugins/cache/debug/sample-plugin/1.2.3-beta+7");
    assert_eq!(
        result,
        PluginInstallOutcome {
            plugin_id: PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap(),
            plugin_version: "1.2.3-beta+7".to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path).unwrap(),
            auth_policy: MarketplacePluginAuthPolicy::OnInstall,
        }
    );
}

#[tokio::test]
async fn install_plugin_writes_marketplace_manifest_fallback_when_missing_plugin_json() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    let plugin_root = repo_root.join("plugins/quality-review");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    fs::create_dir_all(plugin_root.join("skills/thermo-nuclear-code-quality-review")).unwrap();
    fs::write(
        plugin_root.join("skills/thermo-nuclear-code-quality-review/SKILL.md"),
        "review skill",
    )
    .unwrap();
    write_file(
        &plugin_root.join("commands/review.md"),
        "---\ndescription: Review code\n---\nReview the current change.\n",
    );
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "quality-review",
      "description": "Strict code quality review focused on maintainability.",
      "source": "./plugins/quality-review",
      "author": {
        "name": "Byron Grogan"
      },
      "skills": [
        "./skills/thermo-nuclear-code-quality-review"
      ],
      "commands": ["./commands/review.md"],
      "category": "code-review"
    }
  ]
}"#,
    )
    .unwrap();

    let result = test_plugins_manager(tmp.path().to_path_buf())
        .install_plugin(
            &unrestricted_config_layer_stack(),
            PluginInstallRequest {
                plugin_name: "quality-review".to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    repo_root.join(".agents/plugins/marketplace.json"),
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();

    let installed_path = tmp.path().join("plugins/cache/debug/quality-review/local");
    assert_eq!(
        result,
        PluginInstallOutcome {
            plugin_id: PluginId::new("quality-review".to_string(), "debug".to_string()).unwrap(),
            plugin_version: "local".to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path.clone()).unwrap(),
            auth_policy: MarketplacePluginAuthPolicy::OnInstall,
        }
    );
    assert!(!plugin_root.join(".codex-plugin/plugin.json").exists());
    assert!(
        !tmp.path()
            .join("plugins/.marketplace-plugin-source-staging")
            .exists()
    );

    let manifest = crate::manifest::load_plugin_manifest(&installed_path).unwrap();
    assert_eq!(manifest.name, "quality-review");
    assert_eq!(
        manifest.description.as_deref(),
        Some("Strict code quality review focused on maintainability.")
    );
    assert_eq!(
        manifest.paths.skills,
        vec![
            AbsolutePathBuf::try_from(
                installed_path.join("skills/thermo-nuclear-code-quality-review")
            )
            .unwrap()
        ]
    );
    let interface = manifest.interface.expect("fallback interface");
    assert_eq!(interface.developer_name.as_deref(), Some("Byron Grogan"));
    assert_eq!(interface.category.as_deref(), Some("code-review"));
    let fallback_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(installed_path.join(".codex-plugin/plugin.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        fallback_json["author"],
        serde_json::json!({ "name": "Byron Grogan" })
    );
    assert_eq!(fallback_json["category"], "code-review");
    assert_eq!(
        fs::read_to_string(
            installed_path
                .join(".codex-plugin/migrated-command-skills/source-command-review/SKILL.md")
        )
        .unwrap(),
        "---\nname: \"source-command-review\"\ndescription: \"Review code\"\n---\n\n# source-command-review\n\nUse this skill when the user asks to run the migrated source command `review`.\n\n## Command Template\n\nReview the current change.\n"
    );
}

#[tokio::test]
async fn install_plugin_supports_git_subdir_marketplace_sources() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("marketplace");
    let remote_repo = tmp.path().join("remote-plugin-repo");
    let remote_repo_url = url::Url::from_directory_path(&remote_repo)
        .unwrap()
        .to_string();
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    write_plugin(&remote_repo, "plugins/toolkit", "toolkit");
    init_git_repo(&remote_repo);
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        format!(
            r#"{{
  "name": "debug",
  "plugins": [
    {{
      "name": "toolkit",
      "source": {{
        "source": "git-subdir",
        "url": "{remote_repo_url}",
        "path": "plugins/toolkit"
      }}
    }}
  ]
}}"#
        ),
    )
    .unwrap();

    let result = test_plugins_manager(tmp.path().to_path_buf())
        .install_plugin(
            &unrestricted_config_layer_stack(),
            PluginInstallRequest {
                plugin_name: "toolkit".to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    repo_root.join(".agents/plugins/marketplace.json"),
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();

    let installed_path = tmp.path().join("plugins/cache/debug/toolkit/local");
    assert_eq!(
        result,
        PluginInstallOutcome {
            plugin_id: PluginId::new("toolkit".to_string(), "debug".to_string()).unwrap(),
            plugin_version: "local".to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path.clone()).unwrap(),
            auth_policy: MarketplacePluginAuthPolicy::OnInstall,
        }
    );
    assert!(installed_path.join(".codex-plugin/plugin.json").is_file());
}

#[tokio::test]
async fn install_plugin_supports_relative_git_subdir_marketplace_sources() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("marketplace");
    let remote_repo = repo_root.join("remote-plugin-repo");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    write_plugin(&remote_repo, "plugins/toolkit", "toolkit");
    init_git_repo(&remote_repo);
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "toolkit",
      "source": {
        "source": "git-subdir",
        "url": "./remote-plugin-repo",
        "path": "plugins/toolkit"
      }
    }
  ]
}"#,
    )
    .unwrap();

    let result = test_plugins_manager(tmp.path().to_path_buf())
        .install_plugin(
            &unrestricted_config_layer_stack(),
            PluginInstallRequest {
                plugin_name: "toolkit".to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    repo_root.join(".agents/plugins/marketplace.json"),
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();

    let installed_path = tmp.path().join("plugins/cache/debug/toolkit/local");
    assert_eq!(
        result,
        PluginInstallOutcome {
            plugin_id: PluginId::new("toolkit".to_string(), "debug".to_string()).unwrap(),
            plugin_version: "local".to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path.clone()).unwrap(),
            auth_policy: MarketplacePluginAuthPolicy::OnInstall,
        }
    );
    assert!(installed_path.join(".codex-plugin/plugin.json").is_file());
}

#[tokio::test]
async fn uninstall_plugin_removes_cache_and_config_entry() {
    let tmp = tempfile::tempdir().unwrap();
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/local",
        "sample-plugin",
    );
    write_file(
        &tmp.path().join(CONFIG_TOML_FILE),
        r#"[features]
plugins = true

[plugins."sample-plugin@debug"]
enabled = true
"#,
    );

    let manager = test_plugins_manager(tmp.path().to_path_buf());
    manager
        .uninstall_plugin("sample-plugin@debug".to_string())
        .await
        .unwrap();
    manager
        .uninstall_plugin("sample-plugin@debug".to_string())
        .await
        .unwrap();

    assert!(
        !tmp.path()
            .join("plugins/cache/debug/sample-plugin")
            .exists()
    );
    let config = fs::read_to_string(tmp.path().join(CONFIG_TOML_FILE)).unwrap();
    assert!(!config.contains(r#"[plugins."sample-plugin@debug"]"#));
}

#[tokio::test]
async fn list_marketplaces_includes_enabled_state() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "enabled-plugin/local",
        "enabled-plugin",
    );
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "disabled-plugin/local",
        "disabled-plugin",
    );
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "enabled-plugin",
      "source": {
        "source": "local",
        "path": "./enabled-plugin"
      }
    },
    {
      "name": "disabled-plugin",
      "source": {
        "source": "local",
        "path": "./disabled-plugin"
      }
    }
  ]
}"#,
    )
    .unwrap();
    write_file(
        &tmp.path().join(CONFIG_TOML_FILE),
        r#"[features]
plugins = true

[plugins."enabled-plugin@debug"]
enabled = true

[plugins."disabled-plugin@debug"]
enabled = false
"#,
    );

    let config = load_config(tmp.path(), &repo_root).await;
    let marketplaces = test_plugins_manager(tmp.path().to_path_buf())
        .list_marketplaces_for_config(
            &config,
            &[AbsolutePathBuf::try_from(repo_root).unwrap()],
            /*include_openai_curated*/ true,
        )
        .unwrap()
        .marketplaces;

    let marketplace = marketplaces
        .into_iter()
        .find(|marketplace| {
            marketplace.path
                == AbsolutePathBuf::try_from(
                    tmp.path().join("repo/.agents/plugins/marketplace.json"),
                )
                .unwrap()
        })
        .expect("expected repo marketplace entry");

    assert_eq!(
        marketplace,
        ConfiguredMarketplace {
            name: "debug".to_string(),
            path: AbsolutePathBuf::try_from(
                tmp.path().join("repo/.agents/plugins/marketplace.json"),
            )
            .unwrap(),
            interface: None,
            plugins: vec![
                ConfiguredMarketplacePlugin {
                    id: "enabled-plugin@debug".to_string(),
                    name: "enabled-plugin".to_string(),
                    local_version: None,
                    installed_version: Some("local".to_string()),
                    source: MarketplacePluginSource::Local {
                        path: AbsolutePathBuf::try_from(tmp.path().join("repo/enabled-plugin"))
                            .unwrap(),
                    },
                    policy: MarketplacePluginPolicy {
                        installation: MarketplacePluginInstallPolicy::Available,
                        authentication: MarketplacePluginAuthPolicy::OnInstall,
                        products: None,
                    },
                    interface: None,
                    keywords: Vec::new(),
                    manifest_fallback: None,
                    installed: true,
                    enabled: true,
                },
                ConfiguredMarketplacePlugin {
                    id: "disabled-plugin@debug".to_string(),
                    name: "disabled-plugin".to_string(),
                    local_version: None,
                    installed_version: Some("local".to_string()),
                    source: MarketplacePluginSource::Local {
                        path: AbsolutePathBuf::try_from(tmp.path().join("repo/disabled-plugin"),)
                            .unwrap(),
                    },
                    policy: MarketplacePluginPolicy {
                        installation: MarketplacePluginInstallPolicy::Available,
                        authentication: MarketplacePluginAuthPolicy::OnInstall,
                        products: None,
                    },
                    interface: None,
                    keywords: Vec::new(),
                    manifest_fallback: None,
                    installed: true,
                    enabled: false,
                },
            ],
        }
    );
}

#[tokio::test]
async fn list_marketplaces_returns_empty_when_feature_disabled() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "enabled-plugin",
      "source": {
        "source": "local",
        "path": "./enabled-plugin"
      }
    }
  ]
}"#,
    )
    .unwrap();
    write_file(
        &tmp.path().join(CONFIG_TOML_FILE),
        r#"[features]
plugins = false

[plugins."enabled-plugin@debug"]
enabled = true
"#,
    );

    let config = load_config(tmp.path(), &repo_root).await;
    let marketplaces = test_plugins_manager(tmp.path().to_path_buf())
        .list_marketplaces_for_config(
            &config,
            &[AbsolutePathBuf::try_from(repo_root).unwrap()],
            /*include_openai_curated*/ true,
        )
        .unwrap()
        .marketplaces;

    assert_eq!(marketplaces, Vec::new());
}

#[tokio::test]
async fn list_marketplaces_excludes_plugins_with_explicit_empty_products() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    fs::write(
        repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "disabled-plugin",
      "source": {
        "source": "local",
        "path": "./disabled-plugin"
      },
      "policy": {
        "products": []
      }
    },
    {
      "name": "default-plugin",
      "source": {
        "source": "local",
        "path": "./default-plugin"
      }
    }
  ]
}"#,
    )
    .unwrap();
    write_file(
        &tmp.path().join(CONFIG_TOML_FILE),
        r#"[features]
plugins = true
"#,
    );

    let config = load_config(tmp.path(), &repo_root).await;
    let marketplaces = test_plugins_manager(tmp.path().to_path_buf())
        .list_marketplaces_for_config(
            &config,
            &[AbsolutePathBuf::try_from(repo_root).unwrap()],
            /*include_openai_curated*/ true,
        )
        .unwrap()
        .marketplaces;

    let marketplace = marketplaces
        .into_iter()
        .find(|marketplace| {
            marketplace.path
                == AbsolutePathBuf::try_from(
                    tmp.path().join("repo/.agents/plugins/marketplace.json"),
                )
                .unwrap()
        })
        .expect("expected repo marketplace entry");
    assert_eq!(
        marketplace.plugins,
        vec![ConfiguredMarketplacePlugin {
            id: "default-plugin@debug".to_string(),
            name: "default-plugin".to_string(),
            local_version: None,
            installed_version: None,
            source: MarketplacePluginSource::Local {
                path: AbsolutePathBuf::try_from(tmp.path().join("repo/default-plugin")).unwrap(),
            },
            policy: MarketplacePluginPolicy {
                installation: MarketplacePluginInstallPolicy::Available,
                authentication: MarketplacePluginAuthPolicy::OnInstall,
                products: None,
            },
            interface: None,
            keywords: Vec::new(),
            manifest_fallback: None,
            installed: false,
            enabled: false,
        }]
    );
}

#[tokio::test]
async fn read_plugin_for_config_returns_plugins_disabled_when_feature_disabled() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    let marketplace_path =
        AbsolutePathBuf::try_from(repo_root.join(".agents/plugins/marketplace.json")).unwrap();
    fs::write(
        marketplace_path.as_path(),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "enabled-plugin",
      "source": {
        "source": "local",
        "path": "./enabled-plugin"
      }
    }
  ]
}"#,
    )
    .unwrap();
    write_file(
        &tmp.path().join(CONFIG_TOML_FILE),
        r#"[features]
plugins = false

[plugins."enabled-plugin@debug"]
enabled = true
"#,
    );

    let config = load_config(tmp.path(), &repo_root).await;
    let err = test_plugins_manager(tmp.path().to_path_buf())
        .read_plugin_for_config(
            &config,
            &PluginReadRequest {
                plugin_name: "enabled-plugin".to_string(),
                marketplace_path,
            },
        )
        .await
        .unwrap_err();

    assert!(matches!(err, MarketplaceError::PluginsDisabled));
}


#[tokio::test]
async fn read_plugin_for_config_uses_marketplace_manifest_fallback_paths_for_local_source() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    let plugin_root = repo_root.join("sample-plugin");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    write_file(
        &repo_root.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "sample-plugin",
      "source": "./sample-plugin",
      "apps": "./config/custom.app.json",
      "mcpServers": {
        "sample-mcp": {
          "command": "sample-mcp"
        }
      }
    }
  ]
}"#,
    );
    write_file(
        &plugin_root.join("config/custom.app.json"),
        r#"{"apps":{"sample-app":{"id":"connector_sample"}}}"#,
    );
    write_file(
        &tmp.path().join(CONFIG_TOML_FILE),
        r#"[features]
plugins = true
"#,
    );

    let config = load_config(tmp.path(), &repo_root).await;
    let manager = test_plugins_manager(tmp.path().to_path_buf());
    let outcome = manager
        .read_plugin_for_config(
            &config,
            &PluginReadRequest {
                plugin_name: "sample-plugin".to_string(),
                marketplace_path: AbsolutePathBuf::try_from(
                    repo_root.join(".agents/plugins/marketplace.json"),
                )
                .unwrap(),
            },
        )
        .await
        .unwrap();

    assert_eq!(
        outcome.plugin.apps,
        vec![AppConnectorId("connector_sample".to_string())]
    );
    assert_eq!(
        outcome.plugin.mcp_server_names,
        vec!["sample-mcp".to_string()]
    );

    let listed_plugin = manager
        .list_marketplaces_for_config(
            &config,
            &[AbsolutePathBuf::try_from(repo_root.clone()).unwrap()],
            /*include_openai_curated*/ false,
        )
        .unwrap()
        .marketplaces
        .into_iter()
        .find(|marketplace| {
            marketplace.path
                == AbsolutePathBuf::try_from(repo_root.join(".agents/plugins/marketplace.json"))
                    .unwrap()
        })
        .unwrap()
        .plugins
        .into_iter()
        .find(|plugin| plugin.name == "sample-plugin")
        .unwrap();
    let listed_detail = manager
        .read_plugin_detail_for_marketplace_plugin(&config, "debug", listed_plugin)
        .await
        .unwrap();
    assert_eq!(
        listed_detail.apps,
        vec![AppConnectorId("connector_sample".to_string())]
    );
    assert_eq!(
        listed_detail.mcp_server_names,
        vec!["sample-mcp".to_string()]
    );
}

#[tokio::test]
async fn agent_plugin_read_and_tool_suggestions_use_portable_capabilities_only() {
    let tmp = tempfile::tempdir().unwrap();
    let repo_root = tmp.path().join("repo");
    let plugin_root = repo_root.join("agent-plugin");
    fs::create_dir_all(repo_root.join(".git")).unwrap();
    fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
    write_file(
        &repo_root.join(".agents/plugins/marketplace.json"),
        r#"{"name":"debug","plugins":[{"name":"agent-plugin","source":"./agent-plugin"}]}"#,
    );
    write_file(
        &plugin_root.join("plugin.json"),
        r#"{"$schema":"https://agent-plugins.org/schemas/1.0.0/plugin.schema.json","name":"agent.tools"}"#,
    );
    write_file(
        &plugin_root.join("skills/direct/SKILL.md"),
        "---\nname: direct\ndescription: Direct skill\n---\n",
    );
    write_file(
        &plugin_root.join("skills/group/nested/SKILL.md"),
        "---\nname: nested\ndescription: Nested skill\n---\n",
    );
    write_file(
        &plugin_root.join("mcp.json"),
        r#"{"$schema":"https://agent-plugins.org/schemas/1.0.0/mcp.schema.json","mcpServers":{"portable":{"type":"stdio","command":"echo"}}}"#,
    );
    write_file(
        &plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"apps":"./.app.json","hooks":"./hooks/hooks.json"}"#,
    );
    write_file(
        &plugin_root.join(".app.json"),
        r#"{"apps":{"legacy":{"id":"connector_legacy"}}}"#,
    );
    write_file(
        &plugin_root.join("hooks/hooks.json"),
        r#"{"hooks":{"SessionStart":[{"hooks":[{"type":"command","command":"echo legacy"}]}]}}"#,
    );
    write_file(
        &tmp.path().join(CONFIG_TOML_FILE),
        "[features]\nplugins = true\n",
    );

    let config = load_config(tmp.path(), &repo_root).await;
    let manager = test_plugins_manager(tmp.path().to_path_buf());
    let plugin = manager
        .list_marketplaces_for_config(
            &config,
            &[AbsolutePathBuf::try_from(repo_root).unwrap()],
            /*include_openai_curated*/ false,
        )
        .unwrap()
        .marketplaces
        .into_iter()
        .find(|marketplace| marketplace.name == "debug")
        .unwrap()
        .plugins
        .into_iter()
        .find(|plugin| plugin.name == "agent-plugin")
        .unwrap();
    let detail = manager
        .read_plugin_detail_for_marketplace_plugin(&config, "debug", plugin.clone())
        .await
        .unwrap();
    let suggestion = manager
        .tool_suggest_metadata_for_marketplace_plugin(
            "debug",
            &plugin,
            &SkillConfigRules::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        detail
            .skills
            .iter()
            .map(|skill| skill.name.as_str())
            .collect::<Vec<_>>(),
        vec!["agent.tools:direct"]
    );
    assert_eq!(detail.mcp_server_names, vec!["portable"]);
    assert!(detail.apps.is_empty());
    assert!(detail.hooks.is_empty());
    assert!(suggestion.has_skills);
    assert_eq!(suggestion.mcp_server_names, vec!["portable"]);
    assert!(suggestion.app_connector_ids.is_empty());
}

