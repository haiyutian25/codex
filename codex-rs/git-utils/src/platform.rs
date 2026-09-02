use std::path::Path;

use crate::GitToolingError;

#[cfg(unix)]
pub fn create_symlink(
    _source: &Path,
    link_target: &Path,
    destination: &Path,
) -> Result<(), GitToolingError> {
    use std::os::unix::fs::symlink;

    symlink(link_target, destination)?;
    Ok(())
}

#[cfg(not(unix))]
pub fn create_symlink(
    _source: &Path,
    _link_target: &Path,
    _destination: &Path,
) -> Result<(), GitToolingError> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "codex-git symlink support is only implemented for Unix",
    )
    .into())
}
