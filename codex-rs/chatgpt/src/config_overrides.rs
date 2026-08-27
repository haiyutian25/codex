//! Support for `-c key=value` overrides on the apply command.

use clap::ArgAction;
use clap::Parser;
use serde::de::Error as SerdeError;
use toml::Value;

/// CLI option that captures arbitrary configuration overrides specified as
/// `-c key=value`. It intentionally keeps both halves **unparsed** so that the
/// calling code can decide how to interpret the right-hand side.
#[derive(Parser, Debug, Default, Clone)]
pub struct CliConfigOverrides {
    /// Override a configuration value that would otherwise be loaded from
    /// `~/.codex/config.toml`. Use a dotted path (`foo.bar.baz`) to override
    /// nested values. The `value` portion is parsed as TOML. If it fails to
    /// parse as TOML, the raw string is used as a literal.
    ///
    /// Examples:
    ///   - `-c model="o3"`
    ///   - `-c 'sandbox_permissions=["disk-full-read-access"]'`
    ///   - `-c shell_environment_policy.inherit=all`
    #[arg(
        short = 'c',
        long = "config",
        value_name = "key=value",
        action = ArgAction::Append,
        global = true,
    )]
    pub raw_overrides: Vec<String>,
}

impl CliConfigOverrides {
    /// Parse the raw strings captured from the CLI into a list of `(path,
    /// value)` tuples where `value` is a `serde_json::Value`.
    pub fn parse_overrides(&self) -> Result<Vec<(String, Value)>, String> {
        self.raw_overrides
            .iter()
            .map(|s| {
                // Only split on the *first* '=' so values are free to contain
                // the character.
                let mut parts = s.splitn(2, '=');
                let key = match parts.next() {
                    Some(k) => k.trim(),
                    None => return Err("Override missing key".to_string()),
                };
                let value_str = parts
                    .next()
                    .ok_or_else(|| format!("Invalid override (missing '='): {s}"))?
                    .trim();

                if key.is_empty() {
                    return Err(format!("Empty key in override: {s}"));
                }

                // Attempt to parse as TOML. If that fails, treat it as a raw
                // string. This allows convenient usage such as
                // `-c model=o3` without the quotes.
                let value: Value = match parse_toml_value(value_str) {
                    Ok(v) => v,
                    Err(_) => {
                        // Strip leading/trailing quotes if present
                        let trimmed = value_str.trim().trim_matches(|c| c == '"' || c == '\'');
                        Value::String(trimmed.to_string())
                    }
                };

                Ok((canonicalize_override_key(key), value))
            })
            .collect()
    }
}

fn canonicalize_override_key(key: &str) -> String {
    if key == "use_legacy_landlock" {
        "features.use_legacy_landlock".to_string()
    } else {
        key.to_string()
    }
}

fn parse_toml_value(raw: &str) -> Result<Value, toml::de::Error> {
    let wrapped = format!("_x_ = {raw}");
    let table: toml::Table = toml::from_str(&wrapped)?;
    table
        .get("_x_")
        .cloned()
        .ok_or_else(|| SerdeError::custom("missing sentinel key"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_scalar() {
        let v = parse_toml_value("42").expect("parse");
        assert_eq!(v.as_integer(), Some(42));
    }

    #[test]
    fn fails_on_unquoted_string() {
        assert!(parse_toml_value("hello").is_err());
    }

    #[test]
    fn canonicalizes_use_legacy_landlock_alias() {
        let overrides = CliConfigOverrides {
            raw_overrides: vec!["use_legacy_landlock=true".to_string()],
        };
        let parsed = overrides.parse_overrides().expect("parse_overrides");
        assert_eq!(parsed[0].0.as_str(), "features.use_legacy_landlock");
        assert_eq!(parsed[0].1.as_bool(), Some(true));
    }
}
