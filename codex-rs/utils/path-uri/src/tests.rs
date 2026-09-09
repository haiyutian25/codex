use super::*;
use pretty_assertions::assert_eq;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

#[test]
fn native_byte_joins_preserve_foreign_posix_filenames() {
    let base = PathUri::parse("file:///root/%FE/admin").unwrap();
    for (path, expected) in [
        (
            b"../\xff/%2e?#\\".as_slice(),
            "file:///root/%FE/%FF/%252e%3F%23%5C",
        ),
        (
            b"//other/./x/../\xff/.git".as_slice(),
            "file:///other/%FF/.git",
        ),
        (b"../../../../\xff".as_slice(), "file:///%FF"),
        (b"../plain".as_slice(), "file:///root/%FE/plain"),
    ] {
        assert_eq!(base.join_native_bytes(path).unwrap().to_string(), expected);
    }
    assert!(base.join_native_bytes(b"bad\0\xff").is_err());
}

#[test]
fn file_uri_round_trips_an_absolute_path() {
    let path = AbsolutePathBuf::current_dir()
        .expect("current directory")
        .join("a path/file.rs");

    let uri = PathUri::from_abs_path(&path);

    let uri_string = uri.to_string();
    assert!(uri_string.starts_with("file:"));
    assert!(uri_string.ends_with("/a%20path/file.rs"));
    assert_eq!(
        PathUri::parse(&uri_string).expect("serialized URI should parse"),
        uri
    );
    assert_eq!(
        uri.to_abs_path()
            .expect("local file URI should convert to a native path"),
        path
    );
}

#[cfg(unix)]
#[test]
fn non_native_uri_io_conversion_is_invalid_input() {
    let uris = ["file://server/share/file.txt", "file:///C:/workspace"];

    for uri in uris {
        let uri = PathUri::parse(uri).expect("valid file URI");
        let error = uri
            .to_abs_path()
            .expect_err("URI should not be host-native");

        assert_eq!(
            (error.kind(), error.to_string()),
            (
                io::ErrorKind::InvalidInput,
                format!("'{uri}' is invalid on '{}'", std::env::consts::OS),
            )
        );
    }
}

#[test]
fn infers_path_conventions_from_uri_shape() {
    for (uri, expected) in [
        ("file:///", Some(PathConvention::Posix)),
        ("file:///home/alice/src", Some(PathConvention::Posix)),
        ("file:///C:/Users/Alice/src", Some(PathConvention::Posix)),
        // Opaque fallback for POSIX bytes `/tmp/null-\0-\xff-byte`.
        (
            "file:///%00/bad/path/L3RtcC9udWxsLQAt_y1ieXRl",
            Some(PathConvention::Posix),
        ),
        // Opaque fallback without a POSIX byte prefix has no inferable convention.
        ("file:///%00/bad/path/XABcAC4AXABDAE8ATQAxAFwA", None),
        ("file:///%00/bad/path/YQ", None),
    ] {
        let path = PathUri::parse(uri).expect("valid path URI");

        assert_eq!(path.infer_path_convention(), expected, "inferring {uri}");
    }
}

#[test]
fn path_convention_splits_absolute_relative_and_bare_path_text() {
    for (convention, path, expected) in [
        (
            PathConvention::Posix,
            "/usr/local/bin/bash",
            vec!["", "usr", "local", "bin", "bash"],
        ),
        (
            PathConvention::Posix,
            r"tools\pwsh.exe",
            vec![r"tools\pwsh.exe"],
        ),
    ] {
        assert_eq!(convention.path_segments(path).collect::<Vec<_>>(), expected);
    }
}

#[test]
fn inferred_native_path_string_uses_the_inferred_convention() {
    for (uri, expected) in [
        ("file:///home/alice/a%20file.rs", "/home/alice/a file.rs"),
        ("file:///C:/Users/Alice%20Smith/main.rs", "/C:/Users/Alice Smith/main.rs"),
        ("file:///%00/bad/path/YQ", "file:///%00/bad/path/YQ"),
    ] {
        let path = PathUri::parse(uri).expect("valid path URI");

        assert_eq!(
            path.inferred_native_path_string(),
            expected,
            "rendering {uri}"
        );
        assert_eq!(
            LegacyAppPathString::from(path).as_str(),
            expected,
            "rendering typed API path {uri}"
        );
    }
}

#[test]
fn relative_path_from_is_host_independent() {
    for (path, base, expected) in [
        (
            "file:///home/alice/project/src/a%20file.rs",
            "file:///home/alice/project",
            Some("src/a file.rs"),
        ),
        (
            "file:///home/alice/project",
            "file:///home/alice/project/",
            Some(""),
        ),
        (
            "file:///home/alice/project-two/main.rs",
            "file:///home/alice/project",
            None,
        ),
        ("file:///HOME/alice/project", "file:///home", None),
        (
            "file:///home/alice/project/src%2Fmain.rs",
            "file:///home/alice/project",
            None,
        ),
        (
            "file:///C:/project/main.rs",
            "file:///",
            Some("C:/project/main.rs"),
        ),
    ] {
        let path = PathUri::parse(path).expect("valid path URI");
        let base = PathUri::parse(base).expect("valid base URI");

        assert_eq!(
            path.relative_path_from(&base).as_deref(),
            expected,
            "finding {path} relative to {base}"
        );
    }
}

#[test]
fn relative_path_from_treats_fallback_uris_as_opaque() {
    let path = PathUri::parse("file:///%00/bad/path/YQ").expect("valid fallback URI");
    let other = PathUri::parse("file:///%00/bad/path/Yg").expect("valid fallback URI");
    let root = PathUri::parse("file:///").expect("valid root URI");

    assert_eq!(path.relative_path_from(&path), Some(String::new()));
    assert_eq!(path.relative_path_from(&other), None);
    assert_eq!(path.relative_path_from(&root), None);
}

#[cfg(unix)]
#[test]
fn file_uri_falls_back_for_posix_paths_with_null_bytes() {
    let path = PathBuf::from(std::ffi::OsString::from_vec(
        b"/tmp/null-\0-\xff-byte".to_vec(),
    ));
    let path = AbsolutePathBuf::from_absolute_path_checked(path).expect("absolute POSIX path");

    let uri = PathUri::from_abs_path(&path);

    assert_eq!(
        uri,
        PathUri::parse("file:///%00/bad/path/L3RtcC9udWxsLQAt_y1ieXRl")
            .expect("valid fallback URI")
    );
    let json = serde_json::to_string(&uri).expect("fallback URI should serialize");
    let reparsed: PathUri =
        serde_json::from_str(&json).expect("serialized fallback URI should parse");
    assert_eq!(json, r#""file:///%00/bad/path/L3RtcC9udWxsLQAt_y1ieXRl""#);
    assert_eq!(reparsed, uri);
    assert_eq!(
        reparsed.to_abs_path().expect("fallback URI should decode"),
        path
    );
}

#[cfg(unix)]
#[test]
fn ordinary_bad_path_uri_is_not_decoded_as_a_fallback() {
    let path = AbsolutePathBuf::from_absolute_path_checked("/bad/path/L3RtcC9udWxsLQAt_y1ieXRl")
        .expect("absolute POSIX path");
    let uri = PathUri::from_abs_path(&path);

    assert_eq!(uri.to_string(), "file:///bad/path/L3RtcC9udWxsLQAt_y1ieXRl");
    assert_eq!(
        uri.to_abs_path().expect("URI should convert literally"),
        path
    );
}

#[test]
fn malformed_bad_path_uris_are_rejected() {
    for uri in [
        "file:///%00/bad/path/",
        "file:///%00/bad/path/not*base64",
        "file:///%00/bad/path/YQ==",
        "file:///%00/bad/path/YR",
        "file:///%00/bad/path/YQ/extra",
        "file:///%00/other/YQ",
    ] {
        assert_eq!(
            PathUri::parse(uri),
            Err(PathUriParseError::InvalidFileUriPath {
                path: uri.to_string(),
            }),
            "parsing {uri}"
        );
    }
}

#[test]
fn structurally_valid_bad_path_uri_with_invalid_native_payload_fails_conversion() {
    let uri = PathUri::parse("file:///%00/bad/path/YQ")
        .expect("canonical base64 fallback URI should parse");

    assert_eq!(
        uri.to_abs_path()
            .expect_err("relative fallback payload should not convert")
            .kind(),
        io::ErrorKind::InvalidInput
    );
}

#[test]
fn bad_path_uris_are_opaque_to_lexical_operations() {
    let uri = PathUri::parse("file:///%00/bad/path/YQ")
        .expect("canonical base64 fallback URI should parse");
    let other = PathUri::parse("file:///%00/bad/path/Yg")
        .expect("canonical base64 fallback URI should parse");
    let root = PathUri::parse("file:///").expect("valid root URI");

    assert_eq!(uri.basename(), None);
    assert_eq!(uri.parent(), None);
    assert!(uri.starts_with(&uri));
    assert!(!uri.starts_with(&root));
    assert!(!uri.starts_with(&other));
    assert!(!other.starts_with(&uri));
    assert_eq!(uri.join(""), Ok(uri.clone()));
    assert_eq!(
        uri.join("child"),
        Err(PathUriParseError::InvalidFileUriPath {
            path: uri.to_string(),
        })
    );
}

#[test]
fn file_uri_parses_a_posix_path_on_any_host() {
    let uri = PathUri::parse("file:///home/alice/src/main.rs")
        .expect("POSIX file URI should parse on every host");

    assert_eq!(uri.encoded_path(), "/home/alice/src/main.rs");
    assert_eq!(uri.basename(), Some("main.rs".to_string()));
    assert_eq!(uri.to_string(), "file:///home/alice/src/main.rs");
}

#[test]
fn file_uri_preserves_drive_shaped_posix_paths() {
    for (input, expected_path) in [("file:///C:/Project", "/C:/Project"), ("file:///C:", "/C:")] {
        let uri = PathUri::parse(input).expect("file URI should parse");
        let reparsed = PathUri::parse(&uri.to_string()).expect("file URI should reparse");
        assert_eq!(uri.encoded_path(), expected_path);
        assert_eq!(reparsed, uri);
    }
}

#[test]
#[cfg(unix)]
fn file_uri_accepts_non_utf8_posix_paths() {
    let path = PathBuf::from(std::ffi::OsString::from_vec(b"/tmp/non-utf8-\xff".to_vec()));
    let path = AbsolutePathBuf::from_absolute_path_checked(path).expect("absolute POSIX path");

    let uri = PathUri::from_abs_path(&path);
    assert_eq!(
        uri.to_abs_path()
            .expect("URI should convert to native path"),
        path
    );
    assert_eq!(
        PathUri::parse(&uri.to_string()).expect("non-UTF-8 URI should reparse"),
        uri
    );
}

#[test]
fn file_uri_round_trips_literal_percent_characters() {
    let uri = PathUri::parse("file:///tmp/100%25/file").expect("file URI should parse");

    assert_eq!(uri.to_string(), "file:///tmp/100%25/file");
    assert_eq!(uri.encoded_path(), "/tmp/100%25/file");
    assert_eq!(uri.basename(), Some("file".to_string()));
}

#[test]
fn file_uri_retains_unc_authority() {
    let uri = PathUri::parse("file://server/share/src/main.rs").expect("valid file URI");

    assert_eq!(uri.encoded_path(), "/share/src/main.rs");
    assert_eq!(uri.to_string(), "file://server/share/src/main.rs");
}

#[test]
fn file_uri_spelling_aliases_have_one_canonical_form() {
    for input in [
        "FILE:///workspace/src",
        "file:/workspace/src",
        "file://localhost/workspace/src",
        "file://LOCALHOST/workspace/src",
    ] {
        let uri = PathUri::parse(input).expect("file URI alias should parse");
        assert_eq!(uri.to_string(), "file:///workspace/src", "parsing {input}");
    }
}

#[test]
fn unsupported_schemes_are_rejected_at_construction() {
    for (input, expected_scheme) in [
        ("codex-env:///devbox/workspace", "codex-env"),
        ("artifact://store/object-1", "artifact"),
        ("http://example.com/file", "http"),
        ("https://example.com/file", "https"),
        ("ssh://host/workspace", "ssh"),
        ("vscode-remote://ssh-remote+host/workspace", "vscode-remote"),
        ("untitled:Untitled-1", "untitled"),
    ] {
        let error = PathUri::parse(input).expect_err("unsupported schemes should be rejected");

        assert!(
            matches!(
                error,
                PathUriParseError::UnsupportedScheme(scheme) if scheme == expected_scheme
            ),
            "parsing {input}"
        );
    }
}

#[test]
fn path_uri_serializes_as_a_string() {
    let uri: PathUri = "file:///workspace/src/lib.rs"
        .parse()
        .expect("valid file URI");

    let json = serde_json::to_string(&uri).expect("URI should serialize");
    let deserialized: PathUri = serde_json::from_str(&json).expect("URI should deserialize");

    assert_eq!(json, r#""file:///workspace/src/lib.rs""#);
    assert_eq!(deserialized, uri);
}

#[test]
fn path_uri_rejects_native_absolute_paths_during_deserialization() {
    let path = AbsolutePathBuf::current_dir()
        .expect("current directory")
        .join("workspace/src");
    let json = serde_json::to_string(&path).expect("absolute path should serialize");

    serde_json::from_str::<PathUri>(&json)
        .expect_err("native absolute path should not deserialize as a URI");
}

#[test]
fn path_uri_rejects_relative_native_paths() {
    let error =
        PathUri::from_host_native_path("src/lib.rs").expect_err("relative path should be rejected");

    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
}

#[test]
fn path_uri_rejects_relative_strings_during_deserialization() {
    let error = serde_json::from_str::<PathUri>(r#""src/lib.rs""#)
        .expect_err("relative path should be rejected");

    assert!(error.to_string().contains("relative URL without a base"));
}

#[test]
fn unsupported_scheme_is_rejected_during_deserialization() {
    let error = serde_json::from_str::<PathUri>(r#""artifact://store/object-1""#)
        .expect_err("unsupported scheme should fail deserialization");

    assert!(
        error
            .to_string()
            .contains("unsupported path URI scheme `artifact`")
    );
}

#[test]
fn known_path_uris_reject_queries_and_fragments() {
    let query_error =
        PathUri::parse("file:///tmp/file.rs?version=1").expect_err("query should be rejected");
    let fragment_error =
        PathUri::parse("file:///tmp/file.rs#L1").expect_err("fragment should be rejected");

    assert!(matches!(query_error, PathUriParseError::QueryNotAllowed));
    assert!(matches!(
        fragment_error,
        PathUriParseError::FragmentNotAllowed
    ));
}

#[test]
fn path_uris_reject_encoded_null_bytes() {
    assert!(PathUri::parse("file:///tmp/%00").is_err());
}

#[test]
fn encoded_filename_characters_round_trip_without_becoming_uri_metadata() {
    let uri = PathUri::parse("file:///tmp/a%3Fb%23c%25d")
        .expect("encoded filename characters should parse");

    assert_eq!(uri.to_string(), "file:///tmp/a%3Fb%23c%25d");
    assert_eq!(uri.encoded_path(), "/tmp/a%3Fb%23c%25d");
    assert_eq!(uri.basename(), Some("a?b#c%d".to_string()));
}

#[test]
fn double_encoded_separator_remains_filename_text() {
    let uri = PathUri::parse("file:///tmp/a%252Fb")
        .expect("double-encoded separator should parse as filename text");

    assert_eq!(uri.to_string(), "file:///tmp/a%252Fb");
    assert_eq!(uri.encoded_path(), "/tmp/a%252Fb");
    assert_eq!(uri.basename(), Some("a%2Fb".to_string()));
}

#[test]
fn basename_uses_decoded_uri_segments() {
    for (input, expected) in [
        ("file:///", None),
        ("file:///workspace/src/lib.rs", Some("lib.rs")),
        ("file:///workspace/a%20file.rs", Some("a file.rs")),
        ("file:///C:/", Some("C:")),
        ("file://server/share", Some("share")),
    ] {
        let uri = PathUri::parse(input).expect("valid file URI");
        assert_eq!(
            uri.basename(),
            expected.map(str::to_string),
            "basename for {input}"
        );
    }
}

#[test]
fn path_buf_uses_the_inferred_native_spelling() {
    let posix = PathUri::parse("file:///usr/local/bin/bash").expect("POSIX URI");

    assert_eq!(
        posix.to_path_buf(),
        PathBuf::from("/usr/local/bin/bash"),
    );
}

#[test]
fn parent_stops_at_native_path_roots() {
    for (input, expected) in [
        (
            "file:///workspace/src/lib.rs",
            Some("file:///workspace/src"),
        ),
        ("file:///workspace", Some("file:///")),
        ("file:///", None),
        ("file:///C:/Users", Some("file:///C:")),
    ] {
        let uri = PathUri::parse(input).expect("valid file URI");
        let expected = expected.map(|value| PathUri::parse(value).expect("valid expected URI"));
        assert_eq!(uri.parent(), expected, "parent for {input}");
    }
}

#[test]
fn ancestors_include_self_and_stop_at_native_path_roots() {
    for (input, expected) in [(
        "file:///workspace/src",
        vec!["file:///workspace/src", "file:///workspace", "file:///"],
    )] {
        let uri = PathUri::parse(input).expect("valid file URI");
        let ancestors = uri
            .ancestors()
            .map(|path| path.to_string())
            .collect::<Vec<_>>();
        assert_eq!(ancestors, expected, "ancestors for {input}");
    }
}

#[test]
fn join_normalizes_relative_uri_segments() {
    for (base, relative, expected) in [
        (
            "file:///workspace/src",
            "../tests/test.rs",
            "file:///workspace/tests/test.rs",
        ),
        ("file:///", "../../etc", "file:///etc"),
        (
            "file:///workspace",
            "a?b#c%d",
            "file:///workspace/a%3Fb%23c%25d",
        ),
        ("file:///workspace/", "", "file:///workspace/"),
    ] {
        let base = PathUri::parse(base).expect("valid base URI");
        let expected = PathUri::parse(expected).expect("valid expected URI");
        assert_eq!(base.join(relative), Ok(expected), "joining {relative}");
    }
}

#[test]
fn join_descendant_uses_the_base_path_convention() {
    for (base, relative, expected) in [(
        "file:///workspace",
        "docs/../public",
        "file:///workspace/public",
    )] {
        let base = PathUri::parse(base).expect("valid base URI");
        let expected = PathUri::parse(expected).expect("valid expected URI");
        assert_eq!(
            base.join_descendant(relative),
            Ok(expected),
            "joining {relative}"
        );
    }
}

#[test]
fn join_descendant_rejects_non_descendant_paths() {
    for (base, path) in [
        ("file:///workspace", "/workspace/docs"),
        ("file:///workspace", "../outside"),
    ] {
        let base = PathUri::parse(base).expect("valid base URI");
        assert_eq!(
            base.join_descendant(path),
            Err(PathUriParseError::JoinPathMustBeDescendant(
                path.to_string()
            )),
            "joining {path}"
        );
    }
}

#[test]
fn join_replaces_posix_absolute_path() {
    let base = PathUri::parse("file:///workspace").expect("valid base URI");

    assert_eq!(
        base.join("/src"),
        Ok(PathUri::parse("file:///src").expect("valid absolute URI"))
    );
}

#[test]
fn join_keeps_canonicalized_posix_double_slash_paths_hierarchical() {
    let base = PathUri::parse("file:///workspace").expect("valid base URI");
    let cwd = base
        .join("//server/share/project")
        .expect("valid absolute path");

    assert_eq!(
        cwd,
        PathUri::parse("file:///server/share/project").expect("valid canonical URI")
    );
    assert_eq!(
        cwd.parent(),
        Some(PathUri::parse("file:///server/share").expect("valid parent URI"))
    );
    assert_eq!(
        cwd.join("AGENTS.md"),
        Ok(PathUri::parse("file:///server/share/project/AGENTS.md").expect("valid child URI"))
    );
    #[cfg(unix)]
    assert_eq!(
        cwd.to_abs_path()
            .expect("cwd should convert to a native path"),
        AbsolutePathBuf::try_from("/server/share/project")
            .expect("expected native path should be absolute")
    );
}

#[test]
fn join_normalizes_absolute_parent_segments() {
    for (base, path, expected) in [
        ("file:///workspace", "/tmp/a/../b", "file:///tmp/b"),
        ("file:///workspace", "/tmp//a/../b", "file:///tmp/b"),
        ("file:///workspace", "/tmp/a/..//b", "file:///tmp/b"),
        ("file:///workspace", "/tmp/a///../b", "file:///tmp/b"),
        ("file:///workspace", "/tmp/a///b/../..", "file:///tmp"),
    ] {
        let base = PathUri::parse(base).expect("valid base URI");
        let expected = PathUri::parse(expected).expect("valid expected URI");
        assert_eq!(base.join(path), Ok(expected), "joining {path}");
    }
}

#[test]
fn join_absolute_parent_segments_stop_at_native_path_roots() {
    for (base, path, expected) in [
        ("file:///workspace", "/a/..", "file:///"),
        ("file:///workspace", "/../../b", "file:///b"),
    ] {
        let base = PathUri::parse(base).expect("valid base URI");
        let expected = PathUri::parse(expected).expect("valid expected URI");
        assert_eq!(base.join(path), Ok(expected), "joining {path}");
    }
}

#[test]
fn join_collapses_redundant_absolute_separators() {
    for (base, path, expected) in [
        ("file:///workspace", "/tmp///", "file:///tmp/"),
        ("file:///workspace", "///", "file:///"),
        (
            "file:///workspace",
            "///server/share///",
            "file:///server/share/",
        ),
    ] {
        let base = PathUri::parse(base).expect("valid base URI");
        let expected = PathUri::parse(expected).expect("valid expected URI");
        assert_eq!(base.join(path), Ok(expected), "joining {path}");
    }
}

#[test]
fn join_rejects_null_paths() {
    let base = PathUri::parse("file:///workspace").expect("valid base URI");

    assert_eq!(
        base.join("src\0file"),
        Err(PathUriParseError::InvalidFileUriPath {
            path: "src\0file".to_string(),
        })
    );
}

#[test]
fn join_uses_the_base_uri_path_convention() {
    for (base, path, expected) in [(
        "file:///workspace/src",
        "../tests/test.rs",
        "file:///workspace/tests/test.rs",
    )] {
        let base = PathUri::parse(base).expect("valid base URI");
        let expected = PathUri::parse(expected).expect("valid expected URI");
        assert_eq!(base.join(path), Ok(expected), "joining {path}");
    }
}

#[test]
fn starts_with_uses_uri_segment_boundaries() {
    for (path, base, expected) in [
        ("file:///workspace/plugin", "file:///", true),
        ("file:///workspace/plugin", "file:///workspace/plugin", true),
        (
            "file:///workspace/plugin/assets/icon.svg",
            "file:///workspace/plugin",
            true,
        ),
        (
            "file:///workspace/plugin-other/icon.svg",
            "file:///workspace/plugin",
            false,
        ),
        ("file:///WORKSPACE/plugin", "file:///workspace", false),
        (
            "file://other/share/plugins/foo/icon.svg",
            "file://server/share/plugins/foo",
            false,
        ),
        (
            "file:///workspace/plugin/%2F..%2Foutside",
            "file:///workspace/plugin",
            false,
        ),
        (
            "file:///workspace/pri%76ate/file",
            "file:///workspace/%70rivate",
            true,
        ),
        ("file:///workspace/%ff/file", "file:///workspace/%FF", true),
        (
            "file:///workspace/plugin/%5C..%5Coutside",
            "file:///workspace/plugin",
            true,
        ),
    ] {
        let path = PathUri::parse(path).expect("valid path URI");
        let base = PathUri::parse(base).expect("valid base URI");
        assert_eq!(path.starts_with(&base), expected);
    }
}

#[test]
fn overlaps_uses_lexical_containment() {
    for (left, right, expected) in [
        ("file:///workspace", "file:///workspace/src", Some(true)),
        (
            "file:///workspace/src",
            "file:///workspace/tests",
            Some(false),
        ),
        ("file:///WORKSPACE", "file:///workspace/src", Some(false)),
    ] {
        let left = PathUri::parse(left).expect("valid left URI");
        let right = PathUri::parse(right).expect("valid right URI");

        assert_eq!(left.overlaps(&right), expected, "{left} and {right}");
        assert_eq!(right.overlaps(&left), expected, "{right} and {left}");
    }

    let opaque = PathUri::from_opaque_path_bytes(b"/workspace/private");
    let lexical = PathUri::parse("file:///workspace").expect("valid lexical URI");
    assert_eq!(opaque.overlaps(&opaque), Some(true));
    assert_eq!(opaque.overlaps(&lexical), None);
    assert_eq!(lexical.overlaps(&opaque), None);
}

#[test]
fn lexical_depth_counts_validated_nonempty_segments() {
    for (path, expected) in [
        ("file:///", Some(0)),
        ("file:///workspace////", Some(1)),
        ("file:///workspace/%70rivate", Some(2)),
        ("file:///workspace/private%2Fsecret", None),
    ] {
        let path = PathUri::parse(path).expect("valid path URI");
        assert_eq!(path.lexical_depth(), expected, "lexical depth for {path}");
    }

    assert_eq!(
        PathUri::from_opaque_path_bytes(b"/workspace").lexical_depth(),
        None
    );
}

#[test]
fn to_url_returns_the_validated_url() {
    let uri = PathUri::parse("file://localhost/workspace/a%20file.rs").expect("valid file URI");

    assert_eq!(
        uri.to_url(),
        Url::parse("file:///workspace/a%20file.rs").expect("valid URL")
    );
}
