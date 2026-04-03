// ============ IMPORTS ============
use iced_layershell::{application, reexport::{Anchor, KeyboardInteractivity, Layer}, settings::{LayerShellSettings, Settings}, to_layer_message};




// ============ CRATES ============
use crate::ron::{LauncherConfig, load_config};
use crate::subscription::subscription;
use crate::helpers::style::global_style;
use crate::update::update;
use crate::view::view;




// ============ MOD'S ============
mod subscription;
mod helpers;
mod update;
mod tests;
mod color;
mod view;
mod ron;




// ============ STRUCTS/ENUMS, ETC... ============
#[derive(Clone, Debug)]
pub struct AppEntry
{
    pub name:     String,
    pub exec:     String,
    pub comment:  String,
    pub icon:     String,
    pub icon_path: Option<String>,
    pub keywords: Vec<String>,
    pub terminal: bool,
}

#[derive(Default, Clone)]
pub struct AppData
{
    pub query:          String,
    pub entries:        Vec<AppEntry>,
    pub filtered:       Vec<AppEntry>,
    pub selected:       usize,
    pub loading:        bool,
    pub config:         LauncherConfig,
    pub scroll_offset:  f32,
    pub viewport_h:     f32,  // real visible height of the scrollable
    pub content_h:      f32,  // real total content height
    pub copy_feedback:  bool, // true while the "copied" toast is visible
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message
{
    Scrolled(f32, f32, f32),  // offset_y, viewport_h, content_h
    ScrollTo(f32),
    QueryChanged(String),
    EntriesLoaded(Vec<AppEntry>),
    Launch(String),
    CopyToClipboard(String),
    CopiedFeedbackClear,
    SelectUp,
    SelectDown,
    SelectLeft,
    SelectRight,
    Close,
    Nothing,
}




// ============ FUNCTIONS ============
pub fn main() -> Result<(), iced_layershell::Error>
{
    let config = load_config();
    let w = config.window.width;
    let h = config.window.height;
    let init_data = AppData { loading: true, config, ..Default::default() };

    application(move || init_data.clone(), namespace, update, view).subscription(subscription).style(global_style).settings(Settings
    {
        layer_settings: LayerShellSettings
        {
            size:                   Some((w, h)),
            anchor:                 Anchor::empty(),
            layer:                  Layer::Overlay,
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            ..Default::default()
        },
        ..Default::default()
    }).run()
}

fn namespace() -> String { "icelauncher".into() }
