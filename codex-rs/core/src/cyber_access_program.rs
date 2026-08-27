use codex_api::AccessPrograms;
use codex_login::CodexAuth;
use codex_protocol::turn_input::CyberAccessProgram;

pub(crate) fn for_auth(
    auth: Option<&CodexAuth>,
    program: Option<CyberAccessProgram>,
) -> Option<AccessPrograms> {
    // Cyber access programs were bound to ChatGPT account auth, which was
    // removed in this API-key-only build.
    let _ = (auth, program);
    None
}
