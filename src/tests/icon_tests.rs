use crate::helpers::icon::derive_icon_char;

// ── derive_icon_char ─────────────────────────────────────────────────────────

#[test]
fn icon_char_terminal_apps()
{
    assert_eq!(derive_icon_char("Terminal"),  "⊞");
    assert_eq!(derive_icon_char("Alacritty"), "⊞");
    assert_eq!(derive_icon_char("Kitty"),     "⊞");
    assert_eq!(derive_icon_char("Foot"),      "⊞");
}

#[test]
fn icon_char_browser_apps()
{
    assert_eq!(derive_icon_char("Firefox"),        "🌐");
    assert_eq!(derive_icon_char("Chromium"),       "🌐");
    assert_eq!(derive_icon_char("Web Browser"),    "🌐");
    assert_eq!(derive_icon_char("Google Chrome"),  "🌐");
}

#[test]
fn icon_char_music_apps()
{
    assert_eq!(derive_icon_char("Spotify"),   "♫");
    assert_eq!(derive_icon_char("Rhythmbox"), "♫");
    assert_eq!(derive_icon_char("Music"),     "♫");
    assert_eq!(derive_icon_char("MPV"),       "♫");
}

#[test]
fn icon_char_file_manager_apps()
{
    assert_eq!(derive_icon_char("Files"),    "📁");
    assert_eq!(derive_icon_char("Nautilus"), "📁");
    assert_eq!(derive_icon_char("Dolphin"),  "📁");
    assert_eq!(derive_icon_char("Thunar"),   "📁");
}

#[test]
fn icon_char_text_editor_apps()
{
    assert_eq!(derive_icon_char("Text Editor"), "📝");
    assert_eq!(derive_icon_char("Gedit"),       "📝");
    assert_eq!(derive_icon_char("Notepad"),     "📝");
    assert_eq!(derive_icon_char("Kate"),        "📝");
}

#[test]
fn icon_char_image_apps()
{
    assert_eq!(derive_icon_char("Image Viewer"), "🖼");
    assert_eq!(derive_icon_char("Photos"),       "🖼");
    assert_eq!(derive_icon_char("GIMP"),         "🖼");
    assert_eq!(derive_icon_char("Inkscape"),     "🖼");
}

#[test]
fn icon_char_code_editor_apps()
{
    assert_eq!(derive_icon_char("Code"),     "</>");
    assert_eq!(derive_icon_char("VSCode"),   "</>");
    assert_eq!(derive_icon_char("VSCodium"), "</>");
}

#[test]
fn icon_char_settings_apps()
{
    assert_eq!(derive_icon_char("Settings"),        "⚙");
    assert_eq!(derive_icon_char("Control Center"),  "⚙");
    assert_eq!(derive_icon_char("System Prefs"),    "⚙");
}

#[test]
fn icon_char_mail_apps()
{
    assert_eq!(derive_icon_char("Mail"),        "✉");
    assert_eq!(derive_icon_char("Thunderbird"), "✉");
    assert_eq!(derive_icon_char("Email"),       "✉");
}

#[test]
fn icon_char_calculator_apps()
{
    assert_eq!(derive_icon_char("Calculator"), "🧮");
    assert_eq!(derive_icon_char("Math"),       "🧮");
}

#[test]
fn icon_char_unknown_returns_fallback()
{
    assert_eq!(derive_icon_char("Blender"),       "▶");
    assert_eq!(derive_icon_char("Steam"),         "▶");
    assert_eq!(derive_icon_char(""),              "▶");
    assert_eq!(derive_icon_char("RandomAppName"), "▶");
}

#[test]
fn icon_char_case_insensitive()
{
    // All matching is done on lowercased name
    assert_eq!(derive_icon_char("FIREFOX"),  "🌐");
    assert_eq!(derive_icon_char("terminal"), "⊞");
    assert_eq!(derive_icon_char("SPOTIFY"),  "♫");
}
