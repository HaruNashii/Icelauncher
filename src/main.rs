// ============ IMPORTS ============
use iced_layershell::application;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::to_layer_message;




// ============ CRATES ============
use crate::helpers::frecency::FrecencyStore;
use crate::helpers::style::global_style;
use crate::ron::{LauncherConfig, load_config};
use crate::subscription::subscription;
use crate::update::update;
use crate::view::view;




// ============ MOD'S ============
mod color;
mod helpers;
mod ron;
mod subscription;
mod tests;
mod update;
mod view;




// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone, Debug)]
pub struct AppEntry
{
	pub name: String,
	pub exec: String,
	pub comment: String,
	pub icon: String,
	pub icon_path: Option<String>,
	pub keywords: Vec<String>,
	pub terminal: bool,
	pub name_lc: String,
	pub exec_lc: String,
	pub comment_lc: String,
	pub keywords_lc: Vec<String>,
}

#[derive(Default, Clone)]
pub struct AppData
{
	pub alt_pressed: bool,
	pub query: String,
	pub entries: Vec<AppEntry>,
	pub filtered: Vec<AppEntry>,
	pub selected: usize,
	pub loading: bool,
	pub config: LauncherConfig,
	pub scroll_offset: f32,
	pub viewport_h: f32,
	pub content_h: f32,
	pub copy_feedback: bool,
	pub frecency: FrecencyStore,
	pub last_launched: Option<String>,
	pub wl_copy_available: bool,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message
{
	AltPressed(bool),
	Scrolled(f32, f32, f32),
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
	LaunchNth(usize),
	RelaunchLast,
	Close,
	Nothing,
}




// ============ IMPL'S ============
impl AppEntry
{
	pub fn with_normalized(mut self) -> Self
	{
		self.name_lc = self.name.to_lowercase();
		self.exec_lc = self.exec.to_lowercase();
		self.comment_lc = self.comment.to_lowercase();
		self.keywords_lc = self.keywords.iter().map(|k| k.to_lowercase()).collect();
		self
	}
}




// ============ FUNCTIONS ============
pub fn main() -> Result<(), iced_layershell::Error>
{
	let config = load_config();
	let window_width = config.window.width;
	let window_height = config.window.height;
	let frecency = FrecencyStore::load();
	let wl_copy_available = which::which("wl-copy").is_ok();
	let initial_state = AppData {
		loading: true,
		config,
		frecency,
		wl_copy_available,
		..Default::default()
	};
	let layer_settings = LayerShellSettings
	{
		size: Some((window_width, window_height)),
		anchor: Anchor::empty(),
		layer: Layer::Overlay,
		keyboard_interactivity: KeyboardInteractivity::Exclusive,
		..Default::default()
	};
	let settings = Settings { layer_settings, ..Default::default() };

	application(move || initial_state.clone(), app_namespace, update, view)
		.subscription(subscription)
		.style(global_style)
		.settings(settings)
		.run()
}


fn app_namespace() -> String
{
	"icelauncher".into()
}
