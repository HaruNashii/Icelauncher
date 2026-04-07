// ============ IMPORTS ============
use iced::Color;
use iced::Theme;
use iced::Vector;
use iced::border::Radius;
use iced::widget::button::{self, Status};
use iced_layershell::reexport::core::{Border, Shadow};




// ============ CRATES ============
use crate::helpers::color::color_or_gradient;
use crate::ron::LauncherConfig;
use crate::AppData;




// ============ FUNCTIONS ============
pub fn global_style(_app: &AppData, _theme: &Theme) -> iced_layershell::reexport::core::theme::Style
{
	iced_layershell::reexport::core::theme::Style 
        {
		background_color: Color::TRANSPARENT,
		text_color: Color::WHITE,
	}
}


pub fn entry_button_style(status: Status, is_selected: bool, config: &LauncherConfig) -> button::Style
{
	let entry = &config.entry;
	let radius = entry.border_radius;
	let (bg_color, bg_gradient) = entry_background(status, is_selected, config);

	button::Style 
        {
		background: Some(color_or_gradient(bg_gradient, bg_color)),
		text_color: entry.text_color.to_iced(),
		border: Border 
                {
			color: entry_border_color(status, is_selected, config),
			width: entry.border_width,
			radius: Radius 
                        {
				top_left:     radius[0],
				top_right:    radius[1],
				bottom_left:  radius[2],
				bottom_right: radius[3],
			},
		},
		shadow: entry_shadow(status, is_selected, config),
		snap: false,
	}
}


fn entry_background(status: Status, is_selected: bool, config: &LauncherConfig) -> (crate::helpers::color::ColorType, Option<&crate::helpers::color::Gradient>)
{
	let entry = &config.entry;
	match (is_selected, status)
	{
		(true,  Status::Pressed | Status::Hovered) => (entry.selected_hovered_color, entry.selected_hovered_gradient.as_ref()),
		(false, Status::Pressed)                   => (entry.pressed_color,           entry.pressed_gradient.as_ref()),
		(true,  _)                                 => (entry.selected_color,          entry.selected_gradient.as_ref()),
		(false, Status::Hovered)                   => (entry.hovered_color,           entry.hovered_gradient.as_ref()),
		(false, _)                                 => (entry.background_color,        entry.background_gradient.as_ref()),
	}
}


fn entry_border_color(status: Status, is_selected: bool, config: &LauncherConfig) -> iced::Color
{
	let entry = &config.entry;
	match (is_selected, status)
	{
		(true,  _)               => entry.selected_border_color.to_iced(),
		(false, Status::Hovered) => entry.hovered_border_color.to_iced(),
		(false, Status::Pressed) => entry.pressed_border_color.to_iced(),
		_                        => entry.border_color.to_iced(),
	}
}


fn entry_shadow(status: Status, is_selected: bool, config: &LauncherConfig) -> Shadow
{
	let entry = &config.entry;
	let (color, ox, oy, blur) = match (is_selected, status)
	{
		(true,  _)               => (entry.selected_shadow_color, entry.selected_shadow_offset_x, entry.selected_shadow_offset_y, entry.selected_shadow_blur),
		(false, Status::Hovered) => (entry.hovered_shadow_color,  entry.hovered_shadow_offset_x,  entry.hovered_shadow_offset_y,  entry.hovered_shadow_blur),
		_                        => (entry.shadow_color,           entry.shadow_offset_x,           entry.shadow_offset_y,           entry.shadow_blur),
	};
	Shadow { color: color.to_iced(), offset: Vector::new(ox, oy), blur_radius: blur }
}
