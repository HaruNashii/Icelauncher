// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};




// ============ COLOR TYPE ============
/// Accepts three formats in the config file:
///   RGB((255, 255, 255))
///   RGBA((255, 255, 255, 80))    ← alpha 0-100
///   HEX("3d3d3d")               ← 6 or 8 hex digits, no '#' required
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum ColorType
{
    RGB([u32; 3]),
    RGBA([u32; 4]),
    HEX([u8; 9]),
}

impl Default for ColorType { fn default() -> Self { ColorType::RGB([255, 255, 255]) } }

impl<'de> Deserialize<'de> for ColorType
{
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error>
    {
        #[derive(Deserialize)]
        #[allow(clippy::upper_case_acronyms)]
        enum Helper { RGB([u32; 3]), RGBA([u32; 4]), HEX(String) }
        match Helper::deserialize(d)?
        {
            Helper::RGB(v)  => Ok(ColorType::RGB(v)),
            Helper::RGBA(v) => Ok(ColorType::RGBA(v)),
            Helper::HEX(s)  => Ok(hex_color(&s)),
        }
    }
}

impl ColorType
{
    pub fn to_iced(self) -> iced::Color
    {
        match self
        {
            ColorType::RGB([r, g, b])       => iced::Color::from_rgb8(r as u8, g as u8, b as u8),
            ColorType::RGBA([r, g, b, a])   => iced::Color::from_rgba8(r as u8, g as u8, b as u8, (a as f32).clamp(0., 100.) / 100.),
            ColorType::HEX(bytes)           => hex_to_iced(&bytes).unwrap_or(iced::Color::WHITE),
        }
    }
}

fn hex_color(s: &str) -> ColorType
{
    let mut bytes = [0u8; 9];
    let src = s.trim_start_matches('#').as_bytes();
    let len = src.len().min(9);
    bytes[..len].copy_from_slice(&src[..len]);
    ColorType::HEX(bytes)
}

fn hex_to_iced(bytes: &[u8; 9]) -> Option<iced::Color>
{
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(9);
    let s   = std::str::from_utf8(&bytes[..end]).ok()?;
    let hex = s.trim_start_matches('#');
    if hex.len() == 6
    {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(iced::Color::from_rgb8(r, g, b))
    }
    else if hex.len() == 8
    {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some(iced::Color::from_rgba8(r, g, b, a as f32 / 255.))
    }
    else { None }
}




// ============ SUB-CONFIGS ============

/// Window geometry and layer-shell placement
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct WindowConfig
{
    /// Width of the launcher window in pixels
    pub width:               u32,
    /// Height of the launcher window in pixels
    pub height:              u32,
    /// Maximum number of results shown at once (controls scroll area)
    pub max_results:         usize,
    /// Inner padding around all content
    pub padding:             u32,
    /// Spacing between the search bar and the result list
    pub section_spacing:     u32,
    /// Spacing between individual result entries
    pub entry_spacing:       u32,
    /// Corner radius of the outer window panel [top-left, top-right, bottom-left, bottom-right]
    pub border_radius:       [f32; 4],
    /// Border width of the outer window panel
    pub border_width:        f32,
    /// Border color of the outer window panel
    pub border_color:        ColorType,
    /// Background color of the outer window panel
    pub background_color:    ColorType,
    /// Drop-shadow color
    pub shadow_color:        ColorType,
    /// Drop-shadow X offset
    pub shadow_offset_x:     f32,
    /// Drop-shadow Y offset
    pub shadow_offset_y:     f32,
    /// Drop-shadow blur radius
    pub shadow_blur:         f32,
}

impl Default for WindowConfig
{
    fn default() -> Self
    {
        Self
        {
            width:            560,
            height:           480,
            max_results:      12,
            padding:          14,
            section_spacing:  0,
            entry_spacing:    3,
            border_radius:    [10.0, 10.0, 10.0, 10.0],
            border_width:     1.0,
            border_color:     ColorType::HEX(*b"3d3d3d\0\0\0"),
            background_color: ColorType::RGBA([36, 36, 36, 97]),
            shadow_color:     ColorType::RGBA([0, 0, 0, 50]),
            shadow_offset_x:  0.0,
            shadow_offset_y:  4.0,
            shadow_blur:      20.0,
        }
    }
}


/// Search bar appearance and behaviour
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchConfig
{
    /// Placeholder text shown when the input is empty
    pub placeholder:          String,
    /// Font size of the typed text
    pub text_size:            u32,
    /// Inner padding of the text input field
    pub input_padding:        u32,
    /// Bottom margin between the search bar and results
    pub bottom_margin:        f32,
    /// Background color of the input field
    pub background_color:     ColorType,
    /// Border color of the input field (also used for focus ring)
    pub border_color:         ColorType,
    /// Border width of the input field
    pub border_width:         f32,
    /// Corner radius of the input field
    pub border_radius:        [f32; 4],
    /// Color of typed text
    pub text_color:           ColorType,
    /// Color of placeholder text
    pub placeholder_color:    ColorType,
    /// Color of selected text highlight
    pub selection_color:      ColorType,
}

impl Default for SearchConfig
{
    fn default() -> Self
    {
        Self
        {
            placeholder:       "Search applications...".into(),
            text_size:         16,
            input_padding:     10,
            bottom_margin:     8.0,
            background_color:  ColorType::HEX(*b"303030\0\0\0"),
            border_color:      ColorType::HEX(*b"1c71d8\0\0\0"),
            border_width:      1.5,
            border_radius:     [6.0, 6.0, 6.0, 6.0],
            text_color:        ColorType::RGB([255, 255, 255]),
            placeholder_color: ColorType::RGB([179, 179, 179]),
            selection_color:   ColorType::HEX(*b"1c71d8\0\0\0"),
        }
    }
}


/// Result entry row appearance
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct EntryConfig
{
    /// Font size of the application name
    pub name_size:              u32,
    /// Color of the application name
    pub name_color:             ColorType,
    /// Font size of the comment/description
    pub comment_size:           u32,
    /// Color of the comment/description
    pub comment_color:          ColorType,
    /// Maximum comment length before truncation (0 = no limit)
    pub comment_max_chars:      usize,
    /// Whether to show the comment at all
    pub show_comment:           bool,
    /// Padding inside each entry button [top/bottom, left/right]
    pub padding:                [u32; 2],
    /// Normal (idle) background color
    pub background_color:       ColorType,
    /// Hovered background color
    pub hovered_color:          ColorType,
    /// Pressed background color
    pub pressed_color:          ColorType,
    /// Selected (keyboard focus) background color
    pub selected_color:         ColorType,
    /// Selected + hovered background color
    pub selected_hovered_color: ColorType,
    /// Text color (overrides name/comment when pressed)
    pub text_color:             ColorType,
    /// Border color for normal entries
    pub border_color:           ColorType,
    /// Border color for the selected entry
    pub selected_border_color:  ColorType,
    /// Border width
    pub border_width:           f32,
    /// Corner radius
    pub border_radius:          [f32; 4],
    /// Shadow color
    pub shadow_color:           ColorType,
    /// Shadow X offset
    pub shadow_offset_x:        f32,
    /// Shadow Y offset
    pub shadow_offset_y:        f32,
    /// Shadow blur
    pub shadow_blur:            f32,
}

impl Default for EntryConfig
{
    fn default() -> Self
    {
        Self
        {
            name_size:              14,
            name_color:             ColorType::RGB([255, 255, 255]),
            comment_size:           11,
            comment_color:          ColorType::RGB([179, 179, 179]),
            comment_max_chars:      55,
            show_comment:           true,
            padding:                [6, 10],
            background_color:       ColorType::RGBA([0, 0, 0, 0]),
            hovered_color:          ColorType::HEX(*b"3d3d3d\0\0\0"),
            pressed_color:          ColorType::HEX(*b"1c1c1c\0\0\0"),
            selected_color:         ColorType::RGBA([28, 113, 216, 18]),
            selected_hovered_color: ColorType::HEX(*b"2080e8\0\0\0"),
            text_color:             ColorType::RGB([255, 255, 255]),
            border_color:           ColorType::RGBA([0, 0, 0, 0]),
            selected_border_color:  ColorType::RGBA([28, 113, 216, 50]),
            border_width:           1.0,
            border_radius:          [6.0, 6.0, 6.0, 6.0],
            shadow_color:           ColorType::RGBA([0, 0, 0, 50]),
            shadow_offset_x:        0.0,
            shadow_offset_y:        1.0,
            shadow_blur:            3.0,
        }
    }
}


/// Icon badge shown to the left of each entry
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct IconConfig
{
    /// Whether to show the icon badge at all
    pub show:             bool,
    /// Width of the icon badge in pixels
    pub width:            u32,
    /// Height of the icon badge in pixels
    pub height:           u32,
    /// Font size of the icon character
    pub text_size:        u32,
    /// Gap between the icon badge and the text label
    pub gap:              u32,
    /// Normal badge background color
    pub background_color: ColorType,
    /// Selected badge background color (icon turns white automatically)
    pub selected_color:   ColorType,
    /// Normal icon character color
    pub icon_color:       ColorType,
    /// Border color of the badge
    pub border_color:     ColorType,
    /// Border width of the badge
    pub border_width:     f32,
    /// Corner radius of the badge
    pub border_radius:    [f32; 4],
}

impl Default for IconConfig
{
    fn default() -> Self
    {
        Self
        {
            show:             true,
            width:            36,
            height:           36,
            text_size:        18,
            gap:              10,
            background_color: ColorType::HEX(*b"303030\0\0\0"),
            selected_color:   ColorType::HEX(*b"1c71d8\0\0\0"),
            icon_color:       ColorType::HEX(*b"1c71d8\0\0\0"),
            border_color:     ColorType::HEX(*b"3d3d3d\0\0\0"),
            border_width:     1.0,
            border_radius:    [6.0, 6.0, 6.0, 6.0],
        }
    }
}


/// Footer bar at the bottom of the window
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct FooterConfig
{
    /// Whether to show the footer at all
    pub show:              bool,
    /// Whether to show the keyboard hint text on the left
    pub show_hint:         bool,
    /// Custom hint text (leave empty for the built-in default)
    pub hint_text:         String,
    /// Whether to show the result count on the right
    pub show_count:        bool,
    /// Font size of footer text
    pub text_size:         u32,
    /// Color of footer text
    pub text_color:        ColorType,
    /// Top margin of the footer
    pub top_margin:        f32,
}

impl Default for FooterConfig
{
    fn default() -> Self
    {
        Self
        {
            show:       true,
            show_hint:  true,
            hint_text:  String::new(),
            show_count: true,
            text_size:  11,
            text_color: ColorType::RGB([179, 179, 179]),
            top_margin: 6.0,
        }
    }
}


/// Controls which .desktop file fields to search and how
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchBehaviourConfig
{
    /// Search against the app Name field
    pub search_name:        bool,
    /// Search against the Comment field
    pub search_comment:     bool,
    /// Search against the Exec field (program binary name)
    pub search_exec:        bool,
    /// Search against the Keywords field
    pub search_keywords:    bool,
    /// Whether the search is case-sensitive
    pub case_sensitive:     bool,
    /// Close the launcher immediately after launching an app
    pub close_on_launch:    bool,
    /// Terminal emulator command used for terminal apps (e.g. "kitty -e")
    /// Leave empty to ignore Terminal=true entries
    pub terminal_command:   String,
}

impl Default for SearchBehaviourConfig
{
    fn default() -> Self
    {
        Self
        {
            search_name:      true,
            search_comment:   true,
            search_exec:      false,
            search_keywords:  true,
            case_sensitive:   false,
            close_on_launch:  true,
            terminal_command: String::new(),
        }
    }
}


/// Top-level config
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LauncherConfig
{
    pub window:   WindowConfig,
    pub search:   SearchConfig,
    pub entry:    EntryConfig,
    pub icon:     IconConfig,
    pub footer:   FooterConfig,
    pub behaviour: SearchBehaviourConfig,
}




// ============ LOAD / SAVE ============
pub fn config_path() -> PathBuf
{
    home::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/icelauncher/config.ron")
}

/// Load config from disk; if the file is missing it is created with defaults.
/// If the file exists but is malformed, a warning is printed and defaults are used.
pub fn load_config() -> LauncherConfig
{
    let path = config_path();
    if !path.exists()
    {
        eprintln!("[icelauncher] Config not found — writing default to {}", path.display());
        if let Err(e) = write_default_config(&path)
        {
            eprintln!("[icelauncher] Could not write default config: {e}");
        }
        return LauncherConfig::default();
    }

    match fs::read_to_string(&path)
    {
        Ok(text) => match ron::from_str::<LauncherConfig>(&text)
        {
            Ok(cfg) => { eprintln!("[icelauncher] Config loaded from {}", path.display()); cfg }
            Err(e)  => { eprintln!("[icelauncher] Config parse error ({e}) — using defaults"); LauncherConfig::default() }
        },
        Err(e) => { eprintln!("[icelauncher] Could not read config ({e}) — using defaults"); LauncherConfig::default() }
    }
}

fn write_default_config(path: &PathBuf) -> std::io::Result<()>
{
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    let mut file = fs::File::create(path)?;
    file.write_all(DEFAULT_CONFIG_TEXT.as_bytes())?;
    Ok(())
}




// ============ DEFAULT CONFIG TEXT ============
const DEFAULT_CONFIG_TEXT: &str = r#"// =============================================================================
//  icelauncher — configuration file
//  Generated automatically. Edit freely; the app re-reads this on next launch.
//
//  Color formats supported anywhere a color is expected:
//    RGB((255, 255, 255))
//    RGBA((255, 255, 255, 80))   ← alpha is 0-100
//    HEX("3d3d3d")              ← 6-digit or 8-digit hex (no # needed)
// =============================================================================

LauncherConfig
(

    // ─────────────────────────────────────────────────────────────────────────
    //  WINDOW
    // ─────────────────────────────────────────────────────────────────────────
    window:
    (
        width:              560,
        height:             480,
        max_results:        12,
        padding:            14,
        section_spacing:    0,
        entry_spacing:      3,

        border_radius:      (10.0, 10.0, 10.0, 10.0),
        border_width:       1.0,
        border_color:       HEX("3d3d3d"),
        background_color:   RGBA((36, 36, 36, 97)),

        shadow_color:       RGBA((0, 0, 0, 50)),
        shadow_offset_x:    0.0,
        shadow_offset_y:    4.0,
        shadow_blur:        20.0,
    ),


    // ─────────────────────────────────────────────────────────────────────────
    //  SEARCH BAR
    // ─────────────────────────────────────────────────────────────────────────
    search:
    (
        placeholder:        "Search applications...",
        text_size:          16,
        input_padding:      10,
        bottom_margin:      8.0,

        background_color:   HEX("303030"),
        border_color:       HEX("1c71d8"),
        border_width:       1.5,
        border_radius:      (6.0, 6.0, 6.0, 6.0),

        text_color:         RGB((255, 255, 255)),
        placeholder_color:  RGB((179, 179, 179)),
        selection_color:    HEX("1c71d8"),
    ),


    // ─────────────────────────────────────────────────────────────────────────
    //  RESULT ENTRIES
    // ─────────────────────────────────────────────────────────────────────────
    entry:
    (
        name_size:              14,
        name_color:             RGB((255, 255, 255)),

        comment_size:           11,
        comment_color:          RGB((179, 179, 179)),
        comment_max_chars:      55,
        show_comment:           true,

        padding:                (6, 10),

        background_color:       RGBA((0, 0, 0, 0)),
        hovered_color:          HEX("3d3d3d"),
        pressed_color:          HEX("1c1c1c"),
        selected_color:         RGBA((28, 113, 216, 18)),
        selected_hovered_color: HEX("2080e8"),
        text_color:             RGB((255, 255, 255)),

        border_color:           RGBA((0, 0, 0, 0)),
        selected_border_color:  RGBA((28, 113, 216, 50)),
        border_width:           1.0,
        border_radius:          (6.0, 6.0, 6.0, 6.0),

        shadow_color:           RGBA((0, 0, 0, 50)),
        shadow_offset_x:        0.0,
        shadow_offset_y:        1.0,
        shadow_blur:            3.0,
    ),


    // ─────────────────────────────────────────────────────────────────────────
    //  ICON BADGE
    // ─────────────────────────────────────────────────────────────────────────
    icon:
    (
        show:               true,
        width:              36,
        height:             36,
        text_size:          18,
        gap:                10,

        background_color:   HEX("303030"),
        selected_color:     HEX("1c71d8"),
        icon_color:         HEX("1c71d8"),

        border_color:       HEX("3d3d3d"),
        border_width:       1.0,
        border_radius:      (6.0, 6.0, 6.0, 6.0),
    ),


    // ─────────────────────────────────────────────────────────────────────────
    //  FOOTER
    // ─────────────────────────────────────────────────────────────────────────
    footer:
    (
        show:           true,
        show_hint:      true,
        hint_text:      "",        // leave empty for built-in default
        show_count:     true,
        text_size:      11,
        text_color:     RGB((179, 179, 179)),
        top_margin:     6.0,
    ),


    // ─────────────────────────────────────────────────────────────────────────
    //  SEARCH BEHAVIOUR
    // ─────────────────────────────────────────────────────────────────────────
    behaviour:
    (
        search_name:        true,
        search_comment:     true,
        search_exec:        false,
        search_keywords:    true,
        case_sensitive:     false,
        close_on_launch:    true,

        // Set to your terminal command to support Terminal=true .desktop files.
        // Examples: "kitty -e"  |  "alacritty -e"  |  "foot"
        terminal_command:   "",
    ),
)
"#;
