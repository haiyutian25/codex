//! Hook events are append-only across requirements layers. The managed hook
//! directory is different: only one directory is usable, so conflicting values
//! fail closed.

use crate::HookEventsToml;
use crate::ManagedHooksRequirementsToml;
use crate::RequirementSource;
use crate::Sourced;
use std::path::PathBuf;

use super::stack::composition_conflict;
use super::stack::merge_output_source;

const MANAGED_DIR_FIELD_NAME: &str = "hooks.managed_dir";

#[derive(Default)]
pub(super) struct HookMergeState {
    dir_source: Option<RequirementSource>,
}

impl HookMergeState {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn merge(
        &mut self,
        target: &mut Option<Sourced<ManagedHooksRequirementsToml>>,
        incoming: Option<ManagedHooksRequirementsToml>,
        source: &RequirementSource,
    ) -> Result<(), super::stack::RequirementsCompositionError> {
        let Some(mut incoming) = incoming.filter(|value| !value.is_empty()) else {
            return Ok(());
        };
        let Some(existing) = target.as_mut() else {
            if incoming.managed_dir.is_some() {
                self.dir_source = Some(source.clone());
            }
            *target = Some(Sourced::new(incoming, source.clone()));
            return Ok(());
        };

        let mut changed = self.merge_managed_dir(
            &mut existing.value.managed_dir,
            incoming.managed_dir.take(),
            source,
        )?;
        changed |= append_hook_events(&mut existing.value.hooks, incoming.hooks);
        if changed {
            merge_output_source(&mut existing.source, source);
        }
        Ok(())
    }

    fn merge_managed_dir(
        &mut self,
        existing: &mut Option<PathBuf>,
        incoming: Option<PathBuf>,
        incoming_source: &RequirementSource,
    ) -> Result<bool, super::stack::RequirementsCompositionError> {
        let Some(incoming) = incoming else {
            return Ok(false);
        };

        match existing {
            Some(existing_value) if existing_value != &incoming => {
                let existing_source = self
                    .dir_source
                    .clone()
                    .unwrap_or_else(|| incoming_source.clone());
                Err(composition_conflict(
                    MANAGED_DIR_FIELD_NAME.to_string(),
                    existing_source,
                    incoming_source.clone(),
                    format!(
                        "`{}` conflicts with `{}`",
                        existing_value.display(),
                        incoming.display()
                    ),
                ))
            }
            Some(_) => Ok(false),
            None => {
                *existing = Some(incoming);
                self.dir_source
                    .get_or_insert_with(|| incoming_source.clone());
                Ok(true)
            }
        }
    }
}

fn append_hook_events(existing: &mut HookEventsToml, incoming: HookEventsToml) -> bool {
    // Destructure without `..` so new hook events cannot be introduced without
    // deciding whether requirements layer merging should append them.
    let HookEventsToml {
        pre_tool_use,
        permission_request,
        post_tool_use,
        pre_compact,
        post_compact,
        session_start,
        session_end,
        user_prompt_submit,
        subagent_start,
        subagent_stop,
        stop,
        interrupt,
    } = incoming;

    let mut changed = false;
    changed |= append_vec(&mut existing.pre_tool_use, pre_tool_use);
    changed |= append_vec(&mut existing.permission_request, permission_request);
    changed |= append_vec(&mut existing.post_tool_use, post_tool_use);
    changed |= append_vec(&mut existing.pre_compact, pre_compact);
    changed |= append_vec(&mut existing.post_compact, post_compact);
    changed |= append_vec(&mut existing.session_start, session_start);
    changed |= append_vec(&mut existing.session_end, session_end);
    changed |= append_vec(&mut existing.user_prompt_submit, user_prompt_submit);
    changed |= append_vec(&mut existing.subagent_start, subagent_start);
    changed |= append_vec(&mut existing.subagent_stop, subagent_stop);
    changed |= append_vec(&mut existing.stop, stop);
    changed |= append_vec(&mut existing.interrupt, interrupt);
    changed
}

fn append_vec<T>(existing: &mut Vec<T>, mut incoming: Vec<T>) -> bool {
    let changed = !incoming.is_empty();
    existing.append(&mut incoming);
    changed
}
