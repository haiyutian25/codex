use super::ContextualUserFragment;
use super::world_state::PreviousSectionState;
use super::world_state::WorldStateSection;
use codex_protocol::AgentPath;
use codex_protocol::models::ContentItemKind;
use codex_protocol::protocol::CONTEXT_WINDOW_CLOSE_TAG;
use codex_protocol::protocol::CONTEXT_WINDOW_GUIDANCE_CLOSE_TAG;
use codex_protocol::protocol::CONTEXT_WINDOW_GUIDANCE_OPEN_TAG;
use codex_protocol::protocol::CONTEXT_WINDOW_OPEN_TAG;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokenBudgetContext {
    agent_path: AgentPath,
    first_span_id: Uuid,
    previous_span_id: Option<Uuid>,
    span_id: Uuid,
    thread_hint: Option<String>,
}

impl TokenBudgetContext {
    pub(crate) fn new(
        agent_path: AgentPath,
        first_span_id: Uuid,
        previous_span_id: Option<Uuid>,
        span_id: Uuid,
        thread_hint: Option<String>,
    ) -> Self {
        Self {
            agent_path,
            first_span_id,
            previous_span_id,
            span_id,
            thread_hint,
        }
    }
}

impl ContextualUserFragment for TokenBudgetContext {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("token_budget.context_span".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn requires_separate_message(&self) -> bool {
        true
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (CONTEXT_WINDOW_OPEN_TAG, CONTEXT_WINDOW_CLOSE_TAG)
    }

    fn body(&self) -> String {
        let first_span_id = self.first_span_id;
        let span_id = self.span_id;
        let mut lines = vec![
            format!("Agent name: {}", self.agent_path),
            format!("First context span id: {first_span_id}"),
            format!("Current context span id: {span_id}"),
        ];
        if let Some(previous_span_id) = self.previous_span_id {
            lines.push(format!("Previous context span id: {previous_span_id}"));
        }
        if let Some(thread_hint) = &self.thread_hint {
            lines.push(thread_hint.clone());
        }
        format!("\n{}\n", lines.join("\n"))
    }
}

impl WorldStateSection for TokenBudgetContext {
    const ID: &'static str = "context_span";
    type Snapshot = AgentPath;

    fn snapshot(&self) -> Self::Snapshot {
        self.agent_path.clone()
    }

    fn render_diff(
        &self,
        previous: PreviousSectionState<'_, Self::Snapshot>,
    ) -> Option<Box<dyn ContextualUserFragment>> {
        matches!(previous, PreviousSectionState::Known(agent_path) if agent_path != &self.agent_path)
            .then(|| Box::new(self.clone()) as Box<dyn ContextualUserFragment>)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContextSpanGuidance {
    message: String,
}

impl ContextSpanGuidance {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl ContextualUserFragment for ContextSpanGuidance {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("token_budget.context_span_guidance".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (
            CONTEXT_WINDOW_GUIDANCE_OPEN_TAG,
            CONTEXT_WINDOW_GUIDANCE_CLOSE_TAG,
        )
    }

    fn body(&self) -> String {
        format!("\n{}\n", self.message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokenBudgetRemainingContext {
    tokens_left: Option<i64>,
}

impl TokenBudgetRemainingContext {
    pub(crate) fn new(tokens_left: i64) -> Self {
        Self {
            tokens_left: Some(tokens_left),
        }
    }

    pub(crate) fn unknown() -> Self {
        Self { tokens_left: None }
    }
}

impl ContextualUserFragment for TokenBudgetRemainingContext {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("token_budget.remaining_tokens".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("", "")
    }

    fn body(&self) -> String {
        match self.tokens_left {
            Some(tokens_left) => {
                format!("You have {tokens_left} tokens left in this context span.")
            }
            None => "You have unknown tokens left in this context span.".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokenBudgetReminder {
    message: String,
}

impl TokenBudgetReminder {
    pub(crate) fn new(message_template: &str, n_remaining: i64) -> Self {
        Self {
            message: message_template.replace("{n_remaining}", &n_remaining.to_string()),
        }
    }
}

impl ContextualUserFragment for TokenBudgetReminder {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("token_budget.reminder".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("", "")
    }

    fn body(&self) -> String {
        self.message.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AutoCompactFallbackPrompt {
    message: String,
}

impl AutoCompactFallbackPrompt {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl ContextualUserFragment for AutoCompactFallbackPrompt {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("compaction.auto_fallback_prompt".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("", "")
    }

    fn body(&self) -> String {
        self.message.clone()
    }
}
