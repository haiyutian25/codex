use crate::PathConvention;
use crate::PathUri;
use codex_utils_absolute_path::AbsolutePathBuf;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::path::Path;
use thiserror::Error;
use ts_rs::TS;

/// A UTF-8 path for preserving raw path compatibility at the app-server API
/// boundary while Codex migrates to [`PathUri`].
///
/// Supports storing arbitrary strings read from the API and converting to and
/// from [`PathUri`] using an explicitly selected native path convention.
///
/// When converting from [`PathUri`], "native" refers to the supplied
/// [`PathConvention`], which may be foreign to the operating system running
/// this process. The inner string is private so path-producing code must use a
/// path conversion method instead of bypassing the intended conversion
/// boundary. Non-UTF-8 paths are converted to UTF-8 lossily because this API
/// value is serialized as a JSON string.
///
/// Deserialization and [`Self::from_string`] accept any UTF-8 string without
/// interpreting or validating it. Use [`Self::from_string`] when a caller
/// already owns legacy app-server path text and needs to preserve its wire
/// spelling; use [`Self::from_path`], [`Self::from_abs_path`], or
/// [`Self::from_path_uri`] when converting an actual path value. Relative
/// path text remains valid until an operation such as [`Self::to_path_uri`]
/// requires an absolute path.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, TS)]
#[serde(transparent)]
#[ts(type = "string")]
pub struct LegacyAppPathString(String);

impl LegacyAppPathString {
    /// Preserves already-legacy app-server path text without interpreting it
    /// using the current host.
    ///
    /// This is for API-boundary values that are already strings, including
    /// relative or foreign-platform spellings. Callers with a local
    /// [`Path`], [`AbsolutePathBuf`], or [`PathUri`] should use the
    /// corresponding typed constructor instead.
    pub fn from_string(path: impl Into<String>) -> Self {
        Self(path.into())
    }

    /// Preserves path text without interpreting it using the current host.
    pub fn from_path(path: &Path) -> Self {
        Self(path.to_string_lossy().into_owned())
    }

    /// Renders an absolute path using the current host's path convention.
    pub fn from_abs_path(path: &AbsolutePathBuf) -> Self {
        Self::from_path(path.as_path())
    }

    /// Renders a path URI using the requested native path convention.
    ///
    /// Rendering fails when the URI shape does not match the convention, such
    /// as a POSIX path rendered as Windows or a UNC path rendered as POSIX. It
    /// also fails when an opaque fallback does not encode an absolute path for
    /// the convention. Non-UTF-8 segments are rendered lossily, and encoded
    /// separators are emitted as native path text.
    pub fn from_path_uri(
        path: &PathUri,
        convention: PathConvention,
    ) -> Result<Self, LegacyAppPathStringError> {
        if let Some(path_bytes) = path.opaque_fallback_bytes() {
            return render_opaque_fallback(path, &path_bytes, convention).map(Self);
        }
        match convention {
            PathConvention::Posix => render_posix_path(path).map(Self),
        }
    }

    /// Parses this API string as an absolute path using the requested native
    /// path convention and returns its canonical path URI.
    pub fn to_path_uri(
        &self,
        convention: PathConvention,
    ) -> Result<PathUri, LegacyAppPathStringError> {
        PathUri::from_absolute_native_path(&self.0, convention).ok_or_else(|| {
            LegacyAppPathStringError::InvalidNativePath {
                path: self.0.clone(),
                convention: Some(convention),
            }
        })
    }

    /// Parses this API string as an absolute path using the convention inferred from its spelling.
    pub fn to_inferred_path_uri(&self) -> Option<PathUri> {
        PathUri::try_from(self.clone()).ok()
    }

    /// Renders this API path for display in a user interface.
    ///
    /// Absolute paths are normalized using their inferred native convention.
    /// Strings that cannot be interpreted as absolute paths retain their raw
    /// API spelling.
    pub fn render_for_ui(&self) -> String {
        self.to_inferred_path_uri()
            .map(|path| path.inferred_native_path_string())
            .unwrap_or_else(|| self.0.clone())
    }

    /// Parses this API string as a host-native absolute path.
    pub fn to_inferred_abs_path(&self) -> Option<AbsolutePathBuf> {
        AbsolutePathBuf::try_from(self.clone()).ok()
    }

    /// Infers the path convention of an absolute API path from its spelling.
    ///
    /// Relative paths return `None`.
    pub fn infer_absolute_path_convention(&self) -> Option<PathConvention> {
        self.0.starts_with('/').then_some(PathConvention::Posix)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<AbsolutePathBuf> for LegacyAppPathString {
    fn from(path: AbsolutePathBuf) -> Self {
        Self::from_abs_path(&path)
    }
}

impl From<PathUri> for LegacyAppPathString {
    fn from(path: PathUri) -> Self {
        Self(path.inferred_native_path_string())
    }
}

impl TryFrom<LegacyAppPathString> for PathUri {
    type Error = LegacyAppPathStringError;

    fn try_from(path: LegacyAppPathString) -> Result<Self, Self::Error> {
        let Some(convention) = path.infer_absolute_path_convention() else {
            return Err(LegacyAppPathStringError::InvalidNativePath {
                path: path.0,
                convention: None,
            });
        };
        PathUri::from_absolute_native_path(path.as_str(), convention).ok_or({
            LegacyAppPathStringError::InvalidNativePath {
                path: path.0,
                convention: Some(convention),
            }
        })
    }
}

impl TryFrom<LegacyAppPathString> for AbsolutePathBuf {
    type Error = LegacyAppPathStringError;

    fn try_from(path: LegacyAppPathString) -> Result<Self, Self::Error> {
        AbsolutePathBuf::from_absolute_path_checked(path.as_str()).map_err(|_| {
            LegacyAppPathStringError::InvalidNativePath {
                path: path.0,
                convention: None,
            }
        })
    }
}

fn render_opaque_fallback(
    path: &PathUri,
    path_bytes: &[u8],
    convention: PathConvention,
) -> Result<String, LegacyAppPathStringError> {
    let rendered = match convention {
        PathConvention::Posix if path_bytes.starts_with(b"/") => {
            Some(String::from_utf8_lossy(path_bytes).into_owned())
        }
        _ => None,
    };
    rendered.ok_or_else(|| LegacyAppPathStringError::OpaqueFallback {
        path: path.to_string(),
    })
}

impl fmt::Display for LegacyAppPathString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for LegacyAppPathString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl JsonSchema for LegacyAppPathString {
    fn schema_name() -> String {
        "LegacyAppPathString".to_string()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(generator)
    }
}

fn render_posix_path(path: &PathUri) -> Result<String, LegacyAppPathStringError> {
    let url = path.to_url();
    // POSIX file paths do not have a UNC authority, so `file://server/share`
    // cannot be represented as `/share` without losing the server identity.
    if url.host_str().is_some() {
        return Err(incompatible_convention(path, PathConvention::Posix));
    }

    // URI segments are already separated with `/` on every host. Decode each
    // one independently so `file:///a%20dir/file` becomes `/a dir/file`.
    let mut rendered = String::new();
    for segment in path_segments(&url) {
        rendered.push('/');
        rendered.push_str(&decode_native_segment(segment));
    }
    Ok(rendered)
}

fn path_segments(url: &url::Url) -> std::str::Split<'_, char> {
    url.path_segments()
        .unwrap_or_else(|| unreachable!("validated file URLs have path segments"))
}

fn decode_native_segment(segment: &str) -> String {
    // Decode exactly once. Thus `%20` becomes a space and `%252F` becomes the
    // literal text `%2F`, rather than being decoded a second time into `/`.
    let bytes = urlencoding::decode_binary(segment.as_bytes());
    String::from_utf8_lossy(&bytes).into_owned()
}

fn incompatible_convention(path: &PathUri, convention: PathConvention) -> LegacyAppPathStringError {
    LegacyAppPathStringError::IncompatibleConvention {
        path: path.to_string(),
        convention,
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LegacyAppPathStringError {
    #[error("opaque fallback path URI `{path}` cannot be recovered as a native path")]
    OpaqueFallback { path: String },
    #[error("path URI `{path}` cannot be rendered using {convention} path syntax")]
    IncompatibleConvention {
        path: String,
        convention: PathConvention,
    },
    #[error(
        "path `{path}` is not absolute{convention}",
        convention = .convention.map(|convention| format!(" using {convention} path syntax")).unwrap_or_default()
    )]
    InvalidNativePath {
        path: String,
        convention: Option<PathConvention>,
    },
}

#[cfg(test)]
#[path = "api_path_string_tests.rs"]
mod tests;
