// ============ IMPORTS ============
use std::{fs, io::Write, path::PathBuf};
use serde::{Deserialize, Serialize};




// ============ CRATES ============
use crate::color::{ColorType, hex_color};




// ============ STATIC'S/CONST'S ============
const DEFAULT_CONFIG_TEXT: &str = r#"LauncherConfig
(
    window:
    (
        width:              560,
        height:             480,
        max_results:        12,
        padding:            14,
        section_spacing:    0,
        entry_spacing:      3,
        grid_side_items:    1,

        border_radius:      (10.0, 10.0, 10.0, 10.0),
        border_width:       1.0,
        border_color:       HEX("3d3d3d"),
        background_color:   RGBA((36, 36, 36, 97)),

        shadow_color:       RGBA((0, 0, 0, 50)),
        shadow_offset_x:    0.0,
        shadow_offset_y:    4.0,
        shadow_blur:        20.0,
    ),

    scrollbar:
    (
        show:                    true,
        width:                   6,
        margin:                  2,
        scroller_width:          6,
        border_radius:           (3.0, 3.0, 3.0, 3.0),

        rail_color:              RGBA((0, 0, 0, 0)),
        rail_border_color:       RGBA((0, 0, 0, 0)),
        rail_border_width:       0.0,

        scroller_color:          RGBA((100, 100, 100, 60)),
        scroller_hovered_color:  RGBA((130, 130, 130, 80)),
        scroller_dragging_color: RGBA((160, 160, 160, 100)),
        scroller_border_color:   RGBA((0, 0, 0, 0)),
        scroller_border_width:   0.0,
    ),

    search:
    (
        placeholder:               "Search applications...",
        text_size:                 16,
        input_padding:             10,
        bottom_margin:             8.0,

        background_color:          HEX("303030"),
        focused_background_color:  HEX("303030"),
        border_color:              HEX("1c71d8"),
        focused_border_color:      HEX("1c71d8"),
        border_width:              1.5,
        border_radius:             (6.0, 6.0, 6.0, 6.0),

        text_color:                RGB((255, 255, 255)),
        placeholder_color:         RGB((179, 179, 179)),
        selection_color:           HEX("1c71d8"),

        icon:                      "",
        icon_color:                RGB((179, 179, 179)),

        font_weight:               Normal,
        font_style:                Normal,

        position:                  Top,
        orientation:               Horizontal,
        width:                     0,
        height:                    0,
        fixed_x:                   None,
        fixed_y:                   None,
    ),

    entry:
    (
        name_size:              14,
        name_color:             RGB((255, 255, 255)),
        selected_name_color:    RGB((255, 255, 255)),
        hovered_name_color:     RGB((255, 255, 255)),
        name_font_weight:       Normal,
        name_font_style:        Normal,
        name_align:             Left,
        name_max_chars:         55,

        comment_size:           11,
        comment_color:          RGB((179, 179, 179)),
        selected_comment_color: RGB((179, 179, 179)),
        hovered_comment_color:  RGB((179, 179, 179)),
        comment_font_weight:    Normal,
        comment_font_style:     Normal,
        comment_align:          Left,
        comment_max_chars:      55,
        show_comment:           true,
        name_comment_spacing:   2,

        padding:                (6, 10, 6, 10),
        label_position:         Right,
        width:                  0,
        height:                 0,

        background_color:       RGBA((0, 0, 0, 0)),
        hovered_color:          HEX("3d3d3d"),
        pressed_color:          HEX("1c1c1c"),
        selected_color:         RGBA((28, 113, 216, 18)),
        selected_hovered_color: HEX("2080e8"),
        text_color:             RGB((255, 255, 255)),

        border_color:           RGBA((0, 0, 0, 0)),
        selected_border_color:  RGBA((28, 113, 216, 50)),
        hovered_border_color:   RGBA((0, 0, 0, 0)),
        border_width:           1.0,
        border_radius:          (6.0, 6.0, 6.0, 6.0),

        shadow_color:           RGBA((0, 0, 0, 50)),
        shadow_offset_x:        0.0,
        shadow_offset_y:        1.0,
        shadow_blur:            3.0,

        wrap_word:                      false,
        elipsize_instead_of_wrapping:   true,
        ellipsis:                       "...",
    ),

    icon:
    (
        show:                  true,
        use_real_icons:        true,
        width:                 36,
        height:                36,
        text_size:             18,
        gap:                   10,
        padding:               (0, 0),
        opacity:               1.0,
        selected_opacity:      1.0,

        background_color:      HEX("303030"),
        hovered_color:         HEX("303030"),
        selected_color:        HEX("1c71d8"),
        icon_color:            HEX("1c71d8"),
        selected_icon_color:   RGB((255, 255, 255)),
        hovered_icon_color:    HEX("1c71d8"),

        border_color:          HEX("3d3d3d"),
        selected_border_color: HEX("1c71d8"),
        border_width:          1.0,
        border_radius:         (6.0, 6.0, 6.0, 6.0),
    ),

    footer:
    (
        show:             true,
        show_hint:        true,
        hint_text:        "",
        show_count:       true,
        text_size:        11,
        text_color:       RGB((179, 179, 179)),
        hint_color:       RGB((179, 179, 179)),
        count_color:      RGB((179, 179, 179)),
        font_weight:      Normal,
        font_style:       Normal,
        top_margin:       6.0,
        padding:          (0, 0, 0, 0),
        background_color: RGBA((0, 0, 0, 0)),
        border_color:     RGBA((0, 0, 0, 0)),
        border_width:     0.0,
        border_radius:    (0.0, 0.0, 0.0, 0.0),
        position:         Bottom,
        text_orientation: Horizontal,
        width:            0,
        height:           0,
    ),

    behaviour:
    (
        search_name:        true,
        search_comment:     true,
        search_exec:        false,
        search_keywords:    true,
        case_sensitive:     false,
        close_on_launch:    true,

        terminal_command:   "",

        calc_enabled:           true,
        copy_feedback_text:     "Copied!",
        copy_feedback_seconds:  2.0,
    ),

    background_images: [],
)
"#;




// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum TextAlign
{
    #[default]
    Left,
    Center,
    Right,
}

impl TextAlign
{
    pub fn to_iced(&self) -> iced::alignment::Horizontal
    {
        match self 
        {
            TextAlign::Left   => iced::alignment::Horizontal::Left,
            TextAlign::Center => iced::alignment::Horizontal::Center,
            TextAlign::Right  => iced::alignment::Horizontal::Right,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FontWeight
{
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight
{
    pub fn to_iced(&self) -> iced::font::Weight
    {
        match self 
        {
            FontWeight::Thin       => iced::font::Weight::Thin,
            FontWeight::ExtraLight => iced::font::Weight::ExtraLight,
            FontWeight::Light      => iced::font::Weight::Light,
            FontWeight::Normal     => iced::font::Weight::Normal,
            FontWeight::Medium     => iced::font::Weight::Medium,
            FontWeight::Semibold   => iced::font::Weight::Semibold,
            FontWeight::Bold       => iced::font::Weight::Bold,
            FontWeight::ExtraBold  => iced::font::Weight::ExtraBold,
            FontWeight::Black      => iced::font::Weight::Black,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FontStyle
{
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl FontStyle
{
    pub fn to_iced(&self) -> iced::font::Style
    {
        match self 
        {
            FontStyle::Normal  => iced::font::Style::Normal,
            FontStyle::Italic  => iced::font::Style::Italic,
            FontStyle::Oblique => iced::font::Style::Oblique,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum LabelPosition
{
    #[default]
    Right,
    Left,
    Below,
    Above,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ScrollbarConfig
{
    pub show: bool,
    pub width: u32,
    pub margin: u32,
    pub scroller_width: u32,
    pub border_radius: [f32; 4],
    pub rail_color: ColorType,
    pub rail_border_color: ColorType,
    pub rail_border_width: f32,
    pub scroller_color: ColorType,
    pub scroller_hovered_color: ColorType,
    pub scroller_dragging_color: ColorType,
    pub scroller_border_color: ColorType,
    pub scroller_border_width: f32,
}

impl Default for ScrollbarConfig
{
    fn default() -> Self
    {
        Self 
        {
            show: true,
            width: 6,
            margin: 2,
            scroller_width: 6,
            border_radius: [3.0, 3.0, 3.0, 3.0],
            rail_color: ColorType::RGBA([0, 0, 0, 0]),
            rail_border_color: ColorType::RGBA([0, 0, 0, 0]),
            rail_border_width: 0.0,
            scroller_color: ColorType::RGBA([100, 100, 100, 60]),
            scroller_hovered_color: ColorType::RGBA([130, 130, 130, 80]),
            scroller_dragging_color: ColorType::RGBA([160, 160, 160, 100]),
            scroller_border_color: ColorType::RGBA([0, 0, 0, 0]),
            scroller_border_width: 0.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct WindowConfig
{
    pub width: u32,
    pub height: u32,
    pub max_results: usize,
    pub padding: u32,
    pub section_spacing: u32,
    pub entry_spacing: u32,
    pub grid_side_items: usize,
    pub border_radius: [f32; 4],
    pub border_width: f32,
    pub border_color: ColorType,
    pub background_color: ColorType,
    pub shadow_color: ColorType,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,
}

impl Default for WindowConfig
{
    fn default() -> Self
    {
        Self 
        {
            width: 560,
            height: 480,
            max_results: 12,
            padding: 14,
            section_spacing: 0,
            entry_spacing: 3,
            grid_side_items: 1,
            border_radius: [10.0, 10.0, 10.0, 10.0],
            border_width: 1.0,
            border_color: hex_color("3d3d3d"),
            background_color: ColorType::RGBA([36, 36, 36, 97]),
            shadow_color: ColorType::RGBA([0, 0, 0, 50]),
            shadow_offset_x: 0.0,
            shadow_offset_y: 4.0,
            shadow_blur: 20.0,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SearchPosition
{
    #[default]
    Top,
    Bottom,
    TopLeft,
    TopRight,
    Left,
    Right,
    BottomLeft,
    BottomRight,
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SearchOrientation
{
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchConfig
{
    pub placeholder: String,
    pub text_size: u32,
    pub input_padding: u32,
    pub bottom_margin: f32,
    pub background_color: ColorType,
    pub focused_background_color: ColorType,
    pub border_color: ColorType,
    pub focused_border_color: ColorType,
    pub border_width: f32,
    pub border_radius: [f32; 4],
    pub text_color: ColorType,
    pub placeholder_color: ColorType,
    pub selection_color: ColorType,
    pub icon: String,
    pub icon_color: ColorType,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub position: SearchPosition,
    pub orientation: SearchOrientation,
    pub width: u32,
    pub height: u32,
    pub fixed_x: Option<f32>,
    pub fixed_y: Option<f32>,
}

impl Default for SearchConfig
{
    fn default() -> Self
    {
        Self 
        {
            placeholder: "Search applications...".into(),
            text_size: 16,
            input_padding: 10,
            bottom_margin: 8.0,
            background_color: hex_color("303030"),
            focused_background_color: hex_color("303030"),
            border_color: hex_color("1c71d8"),
            focused_border_color: hex_color("1c71d8"),
            border_width: 1.5,
            border_radius: [6.0, 6.0, 6.0, 6.0],
            text_color: ColorType::RGB([255, 255, 255]),
            placeholder_color: ColorType::RGB([179, 179, 179]),
            selection_color: hex_color("1c71d8"),
            icon: String::new(),
            icon_color: ColorType::RGB([179, 179, 179]),
            font_weight: FontWeight::Normal,
            font_style: FontStyle::Normal,
            position: SearchPosition::Top,
            orientation: SearchOrientation::Horizontal,
            width: 0,
            height: 0,
            fixed_x: None,
            fixed_y: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct EntryConfig
{
    pub name_size: u32,
    pub name_color: ColorType,
    pub selected_name_color: ColorType,
    pub hovered_name_color: ColorType,
    pub name_font_weight: FontWeight,
    pub name_font_style: FontStyle,
    pub name_align: TextAlign,
    pub name_max_chars: usize,

    pub comment_size: u32,
    pub comment_color: ColorType,
    pub selected_comment_color: ColorType,
    pub hovered_comment_color: ColorType,
    pub comment_font_weight: FontWeight,
    pub comment_font_style: FontStyle,
    pub comment_align: TextAlign,
    pub comment_max_chars: usize,
    pub show_comment: bool,
    pub name_comment_spacing: u16,

    pub padding: [u32; 4],
    pub label_position: LabelPosition,
    pub width: u32,
    pub height: u32,

    pub background_color: ColorType,
    pub hovered_color: ColorType,
    pub pressed_color: ColorType,
    pub selected_color: ColorType,
    pub selected_hovered_color: ColorType,
    pub text_color: ColorType,

    pub border_color: ColorType,
    pub selected_border_color: ColorType,
    pub hovered_border_color: ColorType,
    pub border_width: f32,
    pub border_radius: [f32; 4],

    pub shadow_color: ColorType,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,

    pub wrap_word: bool,
    pub elipsize_instead_of_wrapping: bool,
    pub ellipsis: String,
}

impl Default for EntryConfig
{
    fn default() -> Self
    {
        Self 
        {
            name_size: 14,
            name_color: ColorType::RGB([255, 255, 255]),
            selected_name_color: ColorType::RGB([255, 255, 255]),
            hovered_name_color: ColorType::RGB([255, 255, 255]),
            name_font_weight: FontWeight::Normal,
            name_font_style: FontStyle::Normal,
            name_align: TextAlign::Left,
            name_max_chars: 55,

            comment_size: 11,
            comment_color: ColorType::RGB([179, 179, 179]),
            selected_comment_color: ColorType::RGB([179, 179, 179]),
            hovered_comment_color: ColorType::RGB([179, 179, 179]),
            comment_font_weight: FontWeight::Normal,
            comment_font_style: FontStyle::Normal,
            comment_align: TextAlign::Left,
            comment_max_chars: 55,
            show_comment: true,
            name_comment_spacing: 2,

            padding: [6, 10, 6, 10],
            label_position: LabelPosition::Right,
            width: 0,
            height: 0,

            background_color: ColorType::RGBA([0, 0, 0, 0]),
            hovered_color: hex_color("3d3d3d"),
            pressed_color: hex_color("1c1c1c"),
            selected_color: ColorType::RGBA([28, 113, 216, 18]),
            selected_hovered_color: hex_color("2080e8"),
            text_color: ColorType::RGB([255, 255, 255]),

            border_color: ColorType::RGBA([0, 0, 0, 0]),
            selected_border_color: ColorType::RGBA([28, 113, 216, 50]),
            hovered_border_color: ColorType::RGBA([0, 0, 0, 0]),
            border_width: 1.0,
            border_radius: [6.0, 6.0, 6.0, 6.0],

            shadow_color: ColorType::RGBA([0, 0, 0, 50]),
            shadow_offset_x: 0.0,
            shadow_offset_y: 1.0,
            shadow_blur: 3.0,

            wrap_word: false,
            elipsize_instead_of_wrapping: true,
            ellipsis: "...".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct IconConfig
{
    pub show: bool,
    pub use_real_icons: bool,
    pub width: u32,
    pub height: u32,
    pub text_size: u32,
    pub gap: u32,
    pub padding: [u32; 2],
    pub opacity: f32,
    pub selected_opacity: f32,
    pub background_color: ColorType,
    pub hovered_color: ColorType,
    pub selected_color: ColorType,
    pub icon_color: ColorType,
    pub selected_icon_color: ColorType,
    pub hovered_icon_color: ColorType,
    pub border_color: ColorType,
    pub selected_border_color: ColorType,
    pub border_width: f32,
    pub border_radius: [f32; 4],
}

impl Default for IconConfig
{
    fn default() -> Self
    {
        Self 
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
            background_color: hex_color("303030"),
            hovered_color: hex_color("303030"),
            selected_color: hex_color("1c71d8"),
            icon_color: hex_color("1c71d8"),
            selected_icon_color: ColorType::RGB([255, 255, 255]),
            hovered_icon_color: hex_color("1c71d8"),
            border_color: hex_color("3d3d3d"),
            selected_border_color: hex_color("1c71d8"),
            border_width: 1.0,
            border_radius: [6.0, 6.0, 6.0, 6.0],
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FooterPosition
{
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FooterOrientation
{
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct FooterConfig
{
    pub show: bool,
    pub show_hint: bool,
    pub hint_text: String,
    pub show_count: bool,
    pub text_size: u32,
    pub text_color: ColorType,
    pub hint_color: ColorType,
    pub count_color: ColorType,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub top_margin: f32,
    pub padding: [u32; 4],
    pub background_color: ColorType,
    pub border_color: ColorType,
    pub border_width: f32,
    pub border_radius: [f32; 4],
    pub position: FooterPosition,
    pub width: u32,
    pub height: u32,
    pub text_orientation: FooterOrientation,
}

impl Default for FooterConfig
{
    fn default() -> Self
    {
        Self 
        {
            show: true,
            show_hint: true,
            hint_text: String::new(),
            show_count: true,
            text_size: 11,
            text_color: ColorType::RGB([179, 179, 179]),
            hint_color: ColorType::RGB([179, 179, 179]),
            count_color: ColorType::RGB([179, 179, 179]),
            font_weight: FontWeight::Normal,
            font_style: FontStyle::Normal,
            top_margin: 6.0,
            padding: [0, 0, 0, 0],
            background_color: ColorType::RGBA([0, 0, 0, 0]),
            border_color: ColorType::RGBA([0, 0, 0, 0]),
            border_width: 0.0,
            border_radius: [0.0, 0.0, 0.0, 0.0],
            position: FooterPosition::Bottom,
            width: 0,
            height: 0,
            text_orientation: FooterOrientation::Horizontal,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchBehaviourConfig
{
    pub search_name: bool,
    pub search_comment: bool,
    pub search_exec: bool,
    pub search_keywords: bool,
    pub case_sensitive: bool,
    pub close_on_launch: bool,
    pub terminal_command: String,
    pub calc_enabled: bool,
    pub copy_feedback_text: String,
    pub copy_feedback_seconds: f32,
}

impl Default for SearchBehaviourConfig
{
    fn default() -> Self
    {
        Self 
        {
            search_name: true,
            search_comment: true,
            search_exec: false,
            search_keywords: true,
            case_sensitive: false,
            close_on_launch: true,
            terminal_command: String::new(),
            calc_enabled: true,
            copy_feedback_text: "Copied!".into(),
            copy_feedback_seconds: 2.0,
        }
    }
}

/// How a background image is scaled to fit its container.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ImageContentFit
{
    #[default]
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

impl ImageContentFit
{
    pub fn to_iced(&self) -> iced::ContentFit
    {
        match self 
        {
            ImageContentFit::Contain   => iced::ContentFit::Contain,
            ImageContentFit::Cover     => iced::ContentFit::Cover,
            ImageContentFit::Fill      => iced::ContentFit::Fill,
            ImageContentFit::None      => iced::ContentFit::None,
            ImageContentFit::ScaleDown => iced::ContentFit::ScaleDown,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BackgroundImage
{
    pub path: String,
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
    pub opacity: f32,
    pub content_fit: ImageContentFit,
}

impl Default for BackgroundImage
{
    fn default() -> Self
    {
        Self 
        {
            path: String::new(),
            x: 0.0,
            y: 0.0,
            width: 0,
            height: 0,
            opacity: 1.0,
            content_fit: ImageContentFit::Contain,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LauncherConfig
{
    pub window: WindowConfig,
    pub scrollbar: ScrollbarConfig,
    pub search: SearchConfig,
    pub entry: EntryConfig,
    pub icon: IconConfig,
    pub footer: FooterConfig,
    pub behaviour: SearchBehaviourConfig,
    pub background_images: Vec<BackgroundImage>,
}




// ============ FUNCTIONS ============
pub fn config_path() -> PathBuf
{
    home::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".config/icelauncher/config.ron")
}



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

    let text = match fs::read_to_string(&path) 
    {
        Ok(t) => t,
        Err(e) => 
        {
            eprintln!("[icelauncher] Could not read config ({e}) — using defaults");
            return LauncherConfig::default();
        }
    };

    match ron::from_str::<LauncherConfig>(&text) 
    {
        Ok(config) => 
        {
            eprintln!("[icelauncher] Config loaded from {}", path.display());
            config
        }
        Err(e) => 
        {
            eprintln!("[icelauncher] Config parse error ({e}) — using defaults");
            LauncherConfig::default()
        }
    }
}



fn write_default_config(path: &PathBuf) -> std::io::Result<()>
{
    if let Some(parent) = path.parent() 
    {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(DEFAULT_CONFIG_TEXT.as_bytes())?;
    Ok(())
}



