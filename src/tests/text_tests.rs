use crate::helpers::text::truncate;

// ── truncate ─────────────────────────────────────────────────────────────────

#[test]
fn truncate_empty_string_returns_empty()
{
    assert_eq!(truncate("", 10), "");
}

#[test]
fn truncate_max_zero_returns_full_string()
{
    // max=0 means "no limit"
    assert_eq!(truncate("hello", 0), "hello");
}

#[test]
fn truncate_string_shorter_than_max_unchanged()
{
    assert_eq!(truncate("hi", 10), "hi");
}

#[test]
fn truncate_string_exactly_max_unchanged()
{
    assert_eq!(truncate("hello", 5), "hello");
}

#[test]
fn truncate_string_one_over_max_gets_ellipsis()
{
    // 6 chars, max 5 → cut to 4 chars + ellipsis
    let result = truncate("abcdef", 5);
    assert!(result.ends_with('…'), "should end with ellipsis, got: {result}");
    assert_eq!(result.chars().count(), 5);
}

#[test]
fn truncate_long_string_gets_ellipsis()
{
    let result = truncate("Hello, World!", 6);
    assert!(result.ends_with('…'));
    assert_eq!(result.chars().count(), 6);
    assert_eq!(result, "Hello…");
}

#[test]
fn truncate_unicode_multibyte_chars_counted_correctly()
{
    // Each emoji is 1 char in Rust's char count
    let s = "😀😁😂😃😄"; // 5 chars
    assert_eq!(truncate(s, 5), s);

    let result = truncate(s, 4);
    assert!(result.ends_with('…'));
    assert_eq!(result.chars().count(), 4);
}

#[test]
fn truncate_max_one_gives_just_ellipsis()
{
    // max=1, string longer → cut 0 chars + ellipsis
    let result = truncate("abc", 1);
    assert_eq!(result, "…");
}

#[test]
fn truncate_max_two_gives_one_char_plus_ellipsis()
{
    let result = truncate("abcde", 2);
    assert_eq!(result, "a…");
}
