use super::*;
use crate::config_toml::ConfigToml;
use pretty_assertions::assert_eq;

#[test]
fn computer_use_config_round_trips() {
    let config: ConfigToml = toml::from_str(
        r#"
[computer_use]
default_app_access = "deny"
"#,
    )
    .expect("computer use config should deserialize");

    let expected = ComputerUseConfigToml {
        default_app_access: Some(AllowDenyRequirementToml::Deny),
    };
    assert_eq!(config.computer_use, Some(expected.clone()));

    let serialized = toml::to_string(&config).expect("computer use config should serialize");
    let reparsed: ConfigToml =
        toml::from_str(&serialized).expect("serialized computer use config should deserialize");
    assert_eq!(reparsed.computer_use, Some(expected));
}
