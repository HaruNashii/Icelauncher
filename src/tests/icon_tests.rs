use crate::{helpers::{color::{ColorType, hex_color}, icon::derive_icon_char}, ron::IconConfig};

// ── derive_icon_char ─────────────────────────────────────────────────────────

fn create_iconconfig() -> IconConfig
{
        IconConfig
        {
            show: true,
            use_real_icons: true,
            width: 36,
            height: 36,
            text_size: 18,
            gap: 10,
            padding: [0, 0],
            opacity: 1.0,
            selected_opacity: 1.0,
            hovered_opacity: 1.0,
            background_color: hex_color("303030"),
            background_gradient: None,
            hovered_color: hex_color("303030"),
            hovered_gradient: None,
            selected_color: hex_color("1c71d8"),
            selected_gradient: None,
            icon_color: hex_color("1c71d8"),
            selected_icon_color: ColorType::RGB([255, 255, 255]),
            hovered_icon_color: hex_color("1c71d8"),
            border_color: hex_color("3d3d3d"),
            selected_border_color: hex_color("1c71d8"),
            hovered_border_color: hex_color("3d3d3d"),
            border_width: 1.0,
            border_radius: [6.0, 6.0, 6.0, 6.0],
            
            generic_icon:               "🎲".to_string(),
            terminal_generic_icon:      "⊞".to_string(),
            browser_generic_icon:       "🌐".to_string(),
            music_player_generic_icon:  "♫".to_string(),
            file_manager_generic_icon:  "📁".to_string(),
            text_editor_generic_icon:   "📝".to_string(),
            media_viewer_generic_icon:  "🖼".to_string(),
            code_generic_icon:          "</>".to_string(),
            mail_generic_icon:          "✉".to_string(),
            calc_generic_icon:          "🧮".to_string(),
            setting_generic_icon:       "⚙".to_string(),
            game_generic_icon:          "🎮".to_string(),
            social_media_generic_icon:  "💬".to_string()
        }
}

#[test]
fn icon_char_terminal_apps()
{
    assert_eq!(derive_icon_char("Terminal", create_iconconfig()), "⊞");
    assert_eq!(derive_icon_char("Alacritty", create_iconconfig()), "⊞");
    assert_eq!(derive_icon_char("Kitty", create_iconconfig()), "⊞");
    assert_eq!(derive_icon_char("Foot", create_iconconfig()), "⊞");
}

#[test]
fn icon_char_browser_apps()
{
    assert_eq!(derive_icon_char("Firefox", create_iconconfig()), "🌐");
    assert_eq!(derive_icon_char("Chromium", create_iconconfig()), "🌐");
    assert_eq!(derive_icon_char("Web Browser", create_iconconfig()), "🌐");
    assert_eq!(derive_icon_char("Google Chrome", create_iconconfig()), "🌐");
}

#[test]
fn icon_char_music_apps()
{
    assert_eq!(derive_icon_char("Spotify", create_iconconfig()), "♫");
    assert_eq!(derive_icon_char("Rhythmbox", create_iconconfig()), "♫");
    assert_eq!(derive_icon_char("Music", create_iconconfig()), "♫");
    assert_eq!(derive_icon_char("MPV", create_iconconfig()), "♫");
}

#[test]
fn icon_char_file_manager_apps()
{
    assert_eq!(derive_icon_char("Files", create_iconconfig()), "📁");
    assert_eq!(derive_icon_char("Nautilus", create_iconconfig()), "📁");
    assert_eq!(derive_icon_char("Dolphin", create_iconconfig()), "📁");
    assert_eq!(derive_icon_char("Thunar", create_iconconfig()), "📁");
}

#[test]
fn icon_char_text_editor_apps()
{
    assert_eq!(derive_icon_char("Text Editor", create_iconconfig()), "📝");
    assert_eq!(derive_icon_char("Gedit", create_iconconfig()), "📝");
    assert_eq!(derive_icon_char("Notepad", create_iconconfig()), "📝");
    assert_eq!(derive_icon_char("Kate", create_iconconfig()), "📝");
}

#[test]
fn icon_char_image_apps()
{
    assert_eq!(derive_icon_char("Image Viewer", create_iconconfig()), "🖼");
    assert_eq!(derive_icon_char("Photos", create_iconconfig()), "🖼");
    assert_eq!(derive_icon_char("GIMP", create_iconconfig()), "🖼");
    assert_eq!(derive_icon_char("Inkscape", create_iconconfig()), "🖼");
}

#[test]
fn icon_char_code_editor_apps()
{
    assert_eq!(derive_icon_char("Code", create_iconconfig()), "</>");
    assert_eq!(derive_icon_char("VSCode", create_iconconfig()), "</>");
    assert_eq!(derive_icon_char("VSCodium", create_iconconfig()), "</>");
}

#[test]
fn icon_char_settings_apps()
{
    assert_eq!(derive_icon_char("Settings", create_iconconfig()), "⚙");
    assert_eq!(derive_icon_char("Control Center", create_iconconfig()), "⚙");
    assert_eq!(derive_icon_char("System Prefs", create_iconconfig()), "⚙");
}

#[test]
fn icon_char_mail_apps()
{
    assert_eq!(derive_icon_char("Mail", create_iconconfig()), "✉");
    assert_eq!(derive_icon_char("Thunderbird", create_iconconfig()), "✉");
    assert_eq!(derive_icon_char("Email", create_iconconfig()), "✉");
}

#[test]
fn icon_char_calculator_apps()
{
    assert_eq!(derive_icon_char("Calculator", create_iconconfig()), "🧮");
    assert_eq!(derive_icon_char("Math", create_iconconfig()), "🧮");
}

#[test]
fn icon_char_unknown_returns_fallback()
{
    assert_eq!(derive_icon_char("Blender", create_iconconfig()), "🎲");
    assert_eq!(derive_icon_char("", create_iconconfig()), "🎲");
    assert_eq!(derive_icon_char("RandomAppName", create_iconconfig()), "🎲");
}

#[test]
fn icon_char_case_insensitive()
{
    // All matching is done on lowercased name
    assert_eq!(derive_icon_char("FIREFOX", create_iconconfig()), "🌐");
    assert_eq!(derive_icon_char("terminal", create_iconconfig()), "⊞");
    assert_eq!(derive_icon_char("SPOTIFY", create_iconconfig()), "♫");
}
