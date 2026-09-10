//! Tests for shell output text conversion.

use super::StreamOutput;
use pretty_assertions::assert_eq;

#[test]
fn test_utf8_shell_output() {
    // UTF-8 output should pass through unchanged.
    assert_eq!(decode_shell_output("пример".as_bytes()), "пример");
}

#[test]
fn test_invalid_bytes_still_fall_back_to_lossy() {
    // Invalid bytes are replaced rather than rejected.
    let bytes = b"\xFF\xFE\xFD";
    assert_eq!(decode_shell_output(bytes), String::from_utf8_lossy(bytes));
}

fn decode_shell_output(bytes: &[u8]) -> String {
    StreamOutput {
        text: bytes.to_vec(),
        truncated_after_lines: None,
    }
    .from_utf8_lossy()
    .text
}
