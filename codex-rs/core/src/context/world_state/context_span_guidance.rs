use super::PreviousSectionState;
use super::WorldStateSection;
use crate::context::ContextSpanGuidance;
use crate::context::ContextualUserFragment;

/// Model-visible guidance for managing the current context span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextSpanGuidanceState {
    message: String,
}

impl ContextSpanGuidanceState {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl WorldStateSection for ContextSpanGuidanceState {
    const ID: &'static str = "context_span_guidance";
    type Snapshot = String;

    fn snapshot(&self) -> Self::Snapshot {
        self.message.clone()
    }

    fn matches_legacy_fragment(role: &str, text: &str) -> bool {
        role == "developer" && ContextSpanGuidance::matches_text(text)
    }

    fn has_retained_fragment_matcher() -> bool {
        true
    }

    fn matches_retained_fragment(role: &str, text: &str) -> bool {
        Self::matches_legacy_fragment(role, text)
    }

    fn render_diff(
        &self,
        previous: PreviousSectionState<'_, Self::Snapshot>,
    ) -> Option<Box<dyn ContextualUserFragment>> {
        if matches!(previous, PreviousSectionState::Known(message) if message == &self.message) {
            return None;
        }

        Some(Box::new(ContextSpanGuidance::new(&self.message)))
    }
}

#[cfg(test)]
#[path = "context_span_guidance_tests.rs"]
mod tests;
