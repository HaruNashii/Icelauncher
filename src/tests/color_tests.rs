use crate::color::{hex_color, hex_to_iced, ColorType};

// ── hex_color ────────────────────────────────────────────────────────────────

#[test]
fn hex_color_six_digit_without_hash()
{
    let c = hex_color("ff8800");
    assert!(matches!(c, ColorType::HEX(_)));
    if let ColorType::HEX(b) = c
    {
        assert_eq!(&b[..6], b"ff8800");
    }
}

#[test]
fn hex_color_six_digit_with_hash()
{
    let c = hex_color("#ff8800");
    if let ColorType::HEX(b) = c
    {
        // strip_prefix('#') removes the hash before storing
        assert_eq!(&b[..6], b"ff8800");
    }
}

#[test]
fn hex_color_eight_digit_rgba()
{
    let c = hex_color("ff8800cc");
    if let ColorType::HEX(b) = c
    {
        assert_eq!(&b[..8], b"ff8800cc");
    }
}

// ── hex_to_iced ──────────────────────────────────────────────────────────────

#[test]
fn hex_to_iced_pure_white()
{
    let mut bytes = [0u8; 9];
    bytes[..6].copy_from_slice(b"ffffff");
    let color = hex_to_iced(&bytes).expect("should parse white");
    assert!((color.r - 1.0).abs() < 0.01);
    assert!((color.g - 1.0).abs() < 0.01);
    assert!((color.b - 1.0).abs() < 0.01);
}

#[test]
fn hex_to_iced_pure_black()
{
    let mut bytes = [0u8; 9];
    bytes[..6].copy_from_slice(b"000000");
    let color = hex_to_iced(&bytes).expect("should parse black");
    assert!(color.r < 0.01);
    assert!(color.g < 0.01);
    assert!(color.b < 0.01);
}

#[test]
fn hex_to_iced_with_leading_hash()
{
    let mut bytes = [0u8; 9];
    bytes[..7].copy_from_slice(b"#ff0000");
    let color = hex_to_iced(&bytes).expect("should parse red with hash");
    assert!((color.r - 1.0).abs() < 0.01);
    assert!(color.g < 0.01);
    assert!(color.b < 0.01);
}

#[test]
fn hex_to_iced_eight_digit_with_alpha()
{
    let mut bytes = [0u8; 9];
    bytes[..8].copy_from_slice(b"ff0000ff");
    let color = hex_to_iced(&bytes).expect("should parse with alpha");
    assert!((color.r - 1.0).abs() < 0.01);
    assert!((color.a - 1.0).abs() < 0.01);
}

#[test]
fn hex_to_iced_half_alpha()
{
    let mut bytes = [0u8; 9];
    bytes[..8].copy_from_slice(b"ffffff80");
    let color = hex_to_iced(&bytes).expect("should parse half alpha");
    // 0x80 = 128, 128/255 ≈ 0.502
    assert!((color.a - 128.0 / 255.0).abs() < 0.01);
}

#[test]
fn hex_to_iced_invalid_returns_none()
{
    let mut bytes = [0u8; 9];
    bytes[..3].copy_from_slice(b"xyz");
    assert!(hex_to_iced(&bytes).is_none());
}

#[test]
fn hex_to_iced_empty_bytes_returns_none()
{
    let bytes = [0u8; 9];
    // All zeros → empty string → doesn't match 6 or 8 chars
    assert!(hex_to_iced(&bytes).is_none());
}

// ── ColorType::to_iced ───────────────────────────────────────────────────────

#[test]
fn color_type_rgb_converts_correctly()
{
    let c = ColorType::RGB([255, 128, 0]);
    let ic = c.to_iced();
    assert!((ic.r - 1.0).abs() < 0.01);
    assert!((ic.g - 128.0 / 255.0).abs() < 0.01);
    assert!(ic.b < 0.01);
}

#[test]
fn color_type_rgba_alpha_clamped_to_0_100_range()
{
    // Alpha is 0-100 in RGBA, mapped to 0.0-1.0
    let full  = ColorType::RGBA([255, 255, 255, 100]);
    let half  = ColorType::RGBA([255, 255, 255, 50]);
    let none  = ColorType::RGBA([255, 255, 255, 0]);
    let over  = ColorType::RGBA([255, 255, 255, 200]); // clamped to 100

    assert!((full.to_iced().a - 1.0).abs() < 0.01);
    assert!((half.to_iced().a - 0.5).abs() < 0.01);
    assert!(none.to_iced().a < 0.01);
    assert!((over.to_iced().a - 1.0).abs() < 0.01);
}

#[test]
fn color_type_hex_roundtrip()
{
    let c = ColorType::HEX(*b"ff0000\0\0\0");
    let ic = c.to_iced();
    assert!((ic.r - 1.0).abs() < 0.01);
    assert!(ic.g < 0.01);
    assert!(ic.b < 0.01);
}

#[test]
fn color_type_hex_invalid_falls_back_to_white()
{
    // Invalid hex → hex_to_iced returns None → fallback WHITE
    let c = ColorType::HEX(*b"zzzzzz\0\0\0");
    let ic = c.to_iced();
    assert!((ic.r - 1.0).abs() < 0.01);
    assert!((ic.g - 1.0).abs() < 0.01);
    assert!((ic.b - 1.0).abs() < 0.01);
}
