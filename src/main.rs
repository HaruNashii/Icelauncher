// ============ IMPORTS ============
use iced_layershell::{application, reexport::{Anchor, KeyboardInteractivity, Layer}, settings::{LayerShellSettings, Settings}, to_layer_message};




// ============ CRATES ============
use crate::ron::{LauncherConfig, load_config};
use crate::subscription::subscription;
use crate::helpers::global_style;
use crate::update::update;
use crate::view::view;




// ============ MOD'S ============
mod subscription;
mod helpers;
mod update;
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
    pub keywords: Vec<String>,
    pub terminal: bool,
}

#[derive(Default, Clone)]
pub struct AppData
{
    pub query:    String,
    pub entries:  Vec<AppEntry>,
    pub filtered: Vec<AppEntry>,
    pub selected: usize,
    pub loading:  bool,
    pub config:   LauncherConfig,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message
{
    QueryChanged(String),
    EntriesLoaded(Vec<AppEntry>),
    Launch(String),
    SelectUp,
    SelectDown,
    Close,
    Nothing,
}




// ============ FUNCTIONS ============
#[tokio::main]
pub async fn main() -> Result<(), iced_layershell::Error>
{
    let config = load_config();

    let w = config.window.width;
    let h = config.window.height;

    let init_data = AppData { loading: true, config, ..Default::default() };

    application(move || init_data.clone(), namespace, update, view)
        .subscription(subscription)
        .style(global_style)
        .settings(Settings
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
        })
        .run()
}

fn namespace() -> String { "icelauncher".into() }
