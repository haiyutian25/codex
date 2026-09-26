use codex_protocol::protocol::TokenUsage;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AutoCompactSpanIds {
    pub(crate) first_span_id: Uuid,
    pub(crate) previous_span_id: Option<Uuid>,
    pub(crate) span_id: Uuid,
}

impl AutoCompactSpanIds {
    pub(crate) fn new_initial() -> Self {
        let span_id = Uuid::now_v7();
        Self {
            first_span_id: span_id,
            previous_span_id: None,
            span_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AutoCompactSpanSnapshot {
    pub(crate) prefill_input_tokens: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AutoCompactSpanPrefill {
    ServerObserved(i64),
    Estimated(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AutoCompactSpan {
    span_number: u64,
    ids: AutoCompactSpanIds,
    new_context_span_requested: bool,
    /// Absolute input-token baseline for the current compaction window.
    ///
    /// `body_after_prefix` subtracts this from later active-context usage. It is
    /// not the growth itself; server-observed usage replaces estimated
    /// resume/recompute baselines when available.
    prefill_input_tokens: Option<AutoCompactSpanPrefill>,
    token_budget_reminder_delivered: bool,
    auto_compact_fallback_delivered: bool,
}

impl AutoCompactSpan {
    pub(super) fn new_with_ids(ids: AutoCompactSpanIds) -> Self {
        Self {
            span_number: 0,
            ids,
            new_context_span_requested: false,
            prefill_input_tokens: None,
            token_budget_reminder_delivered: false,
            auto_compact_fallback_delivered: false,
        }
    }

    pub(super) fn clear_prefill(&mut self) {
        self.prefill_input_tokens = None;
    }

    pub(super) fn span_number(&self) -> u64 {
        self.span_number
    }

    pub(super) fn ids(&self) -> AutoCompactSpanIds {
        self.ids
    }

    pub(super) fn restore(&mut self, span_number: u64, ids: AutoCompactSpanIds) {
        self.span_number = span_number;
        self.ids = ids;
    }

    pub(super) fn advance(&mut self) -> (u64, AutoCompactSpanIds) {
        self.span_number = self.span_number.saturating_add(1);
        self.ids.previous_span_id = Some(self.ids.span_id);
        self.ids.span_id = Uuid::now_v7();
        self.new_context_span_requested = false;
        self.token_budget_reminder_delivered = false;
        self.auto_compact_fallback_delivered = false;
        (self.span_number, self.ids)
    }

    pub(super) fn claim_token_budget_reminder(&mut self) -> bool {
        !std::mem::replace(&mut self.token_budget_reminder_delivered, true)
    }

    pub(super) fn claim_auto_compact_fallback(&mut self) -> bool {
        !std::mem::replace(&mut self.auto_compact_fallback_delivered, true)
    }

    pub(super) fn request_new_context_span(&mut self) {
        self.new_context_span_requested = true;
    }

    pub(super) fn take_new_context_span_request(&mut self) -> bool {
        let requested = self.new_context_span_requested;
        self.new_context_span_requested = false;
        requested
    }

    /// Records the request-input side of the first server usage sample. The
    /// sampled output from that response is body growth and should remain
    /// counted against the scoped auto-compact budget.
    pub(super) fn ensure_server_observed_prefill_from_usage(&mut self, usage: &TokenUsage) {
        if matches!(
            self.prefill_input_tokens,
            Some(AutoCompactSpanPrefill::ServerObserved(_))
        ) {
            return;
        }

        self.prefill_input_tokens = Some(AutoCompactSpanPrefill::ServerObserved(
            usage.input_tokens.max(0),
        ));
    }

    pub(super) fn set_estimated_prefill(&mut self, tokens: i64) {
        if matches!(
            self.prefill_input_tokens,
            Some(AutoCompactSpanPrefill::ServerObserved(_))
        ) {
            return;
        }

        self.prefill_input_tokens = Some(AutoCompactSpanPrefill::Estimated(tokens.max(0)));
    }

    pub(super) fn snapshot(&self) -> AutoCompactSpanSnapshot {
        let prefill_input_tokens = match self.prefill_input_tokens {
            Some(AutoCompactSpanPrefill::ServerObserved(tokens))
            | Some(AutoCompactSpanPrefill::Estimated(tokens)) => Some(tokens),
            None => None,
        };
        AutoCompactSpanSnapshot {
            prefill_input_tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn tracks_prefill_and_window_boundaries() {
        let mut window = AutoCompactSpan::new_with_ids(AutoCompactSpanIds::new_initial());

        assert_eq!(window.span_number(), 0);
        let initial_span_id = window.ids().span_id;
        assert_eq!(initial_span_id.get_version_num(), 7);
        assert_eq!(
            window.ids(),
            AutoCompactSpanIds {
                first_span_id: initial_span_id,
                previous_span_id: None,
                span_id: initial_span_id,
            }
        );
        let first_span_id = initial_span_id;
        let restored_span_id = Uuid::now_v7();
        let restored_previous_span_id = Uuid::now_v7();
        window.restore(
            /*span_number*/ 3,
            AutoCompactSpanIds {
                first_span_id,
                previous_span_id: Some(restored_previous_span_id),
                span_id: restored_span_id,
            },
        );
        assert_eq!(window.span_number(), 3);
        assert_eq!(window.ids().span_id, restored_span_id);
        assert!(window.claim_token_budget_reminder());
        assert!(!window.claim_token_budget_reminder());
        assert!(window.claim_auto_compact_fallback());
        assert!(!window.claim_auto_compact_fallback());
        window.request_new_context_span();
        assert!(window.take_new_context_span_request());
        assert!(!window.take_new_context_span_request());
        window.request_new_context_span();
        let (span_number, ids) = window.advance();
        assert_eq!(span_number, 4);
        assert_eq!(window.span_number(), 4);
        assert_eq!(window.ids(), ids);
        assert_eq!(ids.first_span_id, first_span_id);
        assert_eq!(ids.previous_span_id, Some(restored_span_id));
        assert_eq!(ids.span_id.get_version_num(), 7);
        assert_ne!(ids.span_id, restored_span_id);
        assert!(!window.take_new_context_span_request());
        assert!(window.claim_token_budget_reminder());
        assert!(window.claim_auto_compact_fallback());

        assert_eq!(
            window.snapshot(),
            AutoCompactSpanSnapshot {
                prefill_input_tokens: None,
            }
        );

        window.set_estimated_prefill(/*tokens*/ 150);
        assert_eq!(
            window.snapshot(),
            AutoCompactSpanSnapshot {
                prefill_input_tokens: Some(150),
            }
        );

        window.ensure_server_observed_prefill_from_usage(&TokenUsage {
            input_tokens: 120,
            total_tokens: 170,
            ..Default::default()
        });
        assert_eq!(
            window.snapshot(),
            AutoCompactSpanSnapshot {
                prefill_input_tokens: Some(120),
            }
        );

        window.ensure_server_observed_prefill_from_usage(&TokenUsage {
            input_tokens: 130,
            total_tokens: 180,
            ..Default::default()
        });
        window.set_estimated_prefill(/*tokens*/ 90);
        assert_eq!(
            window.snapshot(),
            AutoCompactSpanSnapshot {
                prefill_input_tokens: Some(120),
            }
        );
    }
}
