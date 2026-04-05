// ============ IMPORTS ============
use iced::Color;
use iced::Theme;
use iced::Vector;
use iced::border::Radius;
use iced::widget::button::{self, Status};
use iced_layershell::reexport::core::{Border, Shadow};




// ============ CRATES ============
use crate::ron::LauncherConfig;
use crate::AppData;




// ============ FUNCTIONS ============
pub fn global_style(_app: &AppData, _theme: &Theme)
-> iced_layershell::reexport::core::theme::Style
{
	iced_layershell::reexport::core::theme::Style {
		background_color: Color::TRANSPARENT,
		text_color: Color::WHITE,
	}
}


pub fn entry_button_style(
	status: Status,
	is_selected: bool,
	config: &LauncherConfig,
) -> button::Style
{
	let entry = &config.entry;
	let radius = entry.border_radius;

	button::Style {
		background: Some(iced::Background::Color(entry_background(status, is_selected, config))),
		text_color: entry.text_color.to_iced(),
		border: Border {
			color: entry_border_color(status, is_selected, config),
			width: entry.border_width,
			radius: Radius {
				top_left: radius[0],
				top_right: radius[1],
				bottom_left: radius[2],
				bottom_right: radius[3],
			},
		},
		shadow: Shadow {
			color: entry.shadow_color.to_iced(),
			offset: Vector::new(entry.shadow_offset_x, entry.shadow_offset_y),
			blur_radius: entry.shadow_blur,
		},
		snap: false,
	}
}


fn entry_background(status: Status, is_selected: bool, config: &LauncherConfig) -> iced::Color
{
	let entry = &config.entry;
	match (is_selected, status)
	{
		(true, Status::Pressed | Status::Hovered) => entry.selected_hovered_color.to_iced(),
		(false, Status::Pressed) => entry.pressed_color.to_iced(),
		(true, _) => entry.selected_color.to_iced(),
		(false, Status::Hovered) => entry.hovered_color.to_iced(),
		(false, _) => entry.background_color.to_iced(),
	}
}


fn entry_border_color(status: Status, is_selected: bool, config: &LauncherConfig) -> iced::Color
{
	let entry = &config.entry;
	match (is_selected, status)
	{
		(true, _) => entry.selected_border_color.to_iced(),
		(false, Status::Hovered) => entry.hovered_border_color.to_iced(),
		_ => entry.border_color.to_iced(),
	}
}
