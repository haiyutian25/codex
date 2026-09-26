use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use std::collections::BTreeMap;

pub(crate) const NEW_CONTEXT_SPAN_TOOL_NAME: &str = "new_context";

pub fn create_new_context_span_tool() -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: NEW_CONTEXT_SPAN_TOOL_NAME.to_string(),
        description: "Start a new context span. Does not clear, reset, or otherwise affect environment state.".to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(BTreeMap::new(), /*required*/ None, Some(false.into())),
        output_schema: None,
    })
}
