use crate::AllowDenyRequirementToml;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ComputerUseConfigToml {
    pub default_app_access: Option<AllowDenyRequirementToml>,
}

#[cfg(test)]
#[path = "computer_use_tests.rs"]
mod tests;
