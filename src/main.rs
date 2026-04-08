// ============ IMPORTS ============
use iced_layershell::application;
use iced_layershell::reexport::{KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;
use iced::Font;




// ============ CRATES ============
use crate::helpers::color::{ConvertedEntriesColor, convert_entry_color};
use crate::helpers::frecency::FrecencyStore;
use crate::helpers::monitor::get_monitor_res;
use crate::helpers::style::global_style;
use crate::helpers::widget::{make_font, make_font_family};
use crate::ron::{anchor_from_config, LauncherConfig, load_config};
use crate::subscription::subscription;
use crate::update::update;
use crate::view::view;




// ============ MOD'S ============
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
	pub generic_name: String,
	pub exec: String,
	pub comment: String,
	pub icon: String,
	pub icon_path: Option<String>,
	pub keywords: Vec<String>,
	pub terminal: bool,
	pub name_lc: String,
	pub generic_name_lc: String,
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
	pub hovered: Option<usize>,
	pub shell_mode: bool,

        pub footer_font: Font,
        pub search_bar_font: Font,
        pub name_font: Font,
        pub comment_font: Font,
        pub converted_entry_color: ConvertedEntriesColor,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message
{
        KeyboardEvent(iced::keyboard::Event),
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
	HoverEntry(usize),
	HoverClear,

	Nothing,
}




// ============ IMPL'S ============
impl AppEntry
{
	pub fn with_normalized(mut self) -> Self
	{
		self.name_lc         = self.name.to_lowercase();
		self.generic_name_lc = self.generic_name.to_lowercase();
		self.exec_lc         = self.exec.to_lowercase();
		self.comment_lc      = self.comment.to_lowercase();
		self.keywords_lc     = self.keywords.iter().map(|k| k.to_lowercase()).collect();
		self
	}
}




// ============ FUNCTIONS ============
pub fn main() -> Result<(), iced_layershell::Error>
{
	let args: Vec<String> = std::env::args().collect();

	if args.iter().any(|a| a == "--version" || a == "-v")
	{
		println!("icelauncher {}", env!("CARGO_PKG_VERSION"));
		return Ok(());
	}

	if args.iter().any(|a| a == "--help" || a == "-h")
	{
		println!("icelauncher — a Wayland application launcher\n");
		println!("USAGE:");
		println!("  icelauncher [OPTIONS]\n");
		println!("OPTIONS:");
		println!("  -s, --shell    Search and run shell commands instead of .desktop apps");
		println!("  -h, --help     Print this help message and exit");
		println!("  -v, --version  Print version and exit");
		return Ok(());
	}

	let shell_mode  = args.iter().any(|a| a == "--shell" || a == "-s");
	let mut config  = load_config();
	let window_width    = config.window.width;
	let window_height   = config.window.height;
	let frecency    = FrecencyStore::load();
	let wl_copy_available   = which::which("wl-copy").is_ok();
	let anchor  = anchor_from_config(&config.window.anchor);
        let display = config.window.display.clone();
	let margins = 
        (
		config.window.margin_top    as i32,
		config.window.margin_right  as i32,
		config.window.margin_bottom as i32,
		config.window.margin_left   as i32,
	);

        let start_mode = match &display
        { 
            Some(output) => StartMode::TargetScreen(output.to_string()),
            None => StartMode::Active 
        };

        let (mw, mh) = get_monitor_res(display);
        let n_window_size = match (window_width, window_height)
        {
            (0, 0) => Some((mw, mh)),
            (0, h) => Some((mw, h)),
            (w, 0) => Some((w, mh)),
            (w, h) => Some((w, h)),
        };

        if let Some(window_size) = n_window_size
        {
            config.window.width = window_size.0;
            config.window.height = window_size.1;
        };

        let entry_config = &config.entry;
        let name_font    = make_font_family(&entry_config.name_font_weight,    &entry_config.name_font_style,    &entry_config.name_font_family);
        let comment_font = make_font_family(&entry_config.comment_font_weight, &entry_config.comment_font_style, &entry_config.comment_font_family);
        let entries_colors = convert_entry_color(entry_config);

        let search_config = &config.search;
        let search_bar_font = make_font_family(&search_config.font_weight, &search_config.font_style, &search_config.font_family);

        let footer_config = &config.footer;
        let footer_font       = make_font(&footer_config.font_weight, &footer_config.font_style);

	let initial_state = AppData 
        {
		loading: true,
		config,
		frecency,
		wl_copy_available,
		shell_mode,
                search_bar_font,
                name_font,
                comment_font,
                footer_font,
                converted_entry_color: entries_colors,
		..Default::default()
	};

	let layer_settings = LayerShellSettings
	{
		size:                   n_window_size,
		anchor,
		layer:                  Layer::Overlay,
		keyboard_interactivity: KeyboardInteractivity::Exclusive,
		margin:                 margins,
                start_mode,
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
