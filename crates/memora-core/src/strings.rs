// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

//! String-manipulation helpers shared across core crates.

/// Safely slices `s` at `max_bytes` without splitting a UTF-8 code point.
///
/// If `max_bytes` lands inside a multi-byte character sequence, the slice is
/// truncated to the last valid code-point boundary before `max_bytes`.
pub fn safe_byte_prefix(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// Owned, char-aware truncation. If `s` has more than `max_chars` Unicode
/// scalar values, returns the first `max_chars` chars followed by an
/// ellipsis (`…`). Otherwise returns `s` unchanged as an owned `String`.
pub fn truncate_string(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max_chars).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_byte_prefix_returns_original_when_short() {
        assert_eq!(safe_byte_prefix("hi", 10), "hi");
    }

    #[test]
    fn safe_byte_prefix_snaps_down_inside_multibyte() {
        // '…' is 3 bytes (E2 80 A6). Asking for 1 or 2 bytes would land
        // mid-codepoint with naked slicing; helper snaps back to 0.
        let s = "a…b";
        assert_eq!(safe_byte_prefix(s, 1), "a");
        assert_eq!(safe_byte_prefix(s, 2), "a");
        assert_eq!(safe_byte_prefix(s, 3), "a");
        assert_eq!(safe_byte_prefix(s, 4), "a…");
    }

    #[test]
    fn safe_byte_prefix_handles_emoji() {
        // 4-byte codepoint.
        let s = "x🎉y";
        assert_eq!(safe_byte_prefix(s, 1), "x");
        assert_eq!(safe_byte_prefix(s, 4), "x");
        assert_eq!(safe_byte_prefix(s, 5), "x🎉");
    }

    #[test]
    fn safe_byte_prefix_zero() {
        assert_eq!(safe_byte_prefix("anything", 0), "");
    }

    #[test]
    fn truncate_string_keeps_short_input() {
        assert_eq!(truncate_string("hi", 10), "hi");
    }

    #[test]
    fn truncate_string_counts_chars_not_bytes() {
        // 5 multi-byte chars; should keep 3 then ellipsis.
        let s = "αβγδε";
        assert_eq!(truncate_string(s, 3), "αβγ…");
    }

    #[test]
    fn truncate_string_handles_emoji_boundary() {
        // Emoji count as one char each — must not split.
        assert_eq!(truncate_string("a🎉b🎉c", 3), "a🎉b…");
    }
}
