// ============ IMPORTS ============
use iced::{Vector, Color, Theme};
use iced::border::Radius;
use iced::widget::button::{self, Status};
use iced_layershell::reexport::core::{Shadow, Border};




// ============ CRATES ============
use crate::AppData;
use crate::ron::LauncherConfig;




// ============ FUNCTIONS ============
pub fn global_style(_app: &AppData, _theme: &Theme) -> iced_layershell::reexport::core::theme::Style
{
    iced_layershell::reexport::core::theme::Style
    {
        background_color: Color::TRANSPARENT,
        text_color:       Color::WHITE,
    }
}



pub fn entry_button_style(status: Status, is_selected: bool, cfg: &LauncherConfig) -> button::Style
{
    let ec = &cfg.entry;

    let bg = match (is_selected, status)
    {
        (true,  Status::Pressed | Status::Hovered)  => ec.selected_hovered_color.to_iced(),
        (false, Status::Pressed)  => ec.pressed_color.to_iced(),
        (true, _)                 => ec.selected_color.to_iced(),
        (false, Status::Hovered)  => ec.hovered_color.to_iced(),
        (false, _)                => ec.background_color.to_iced(),
    };

    let border_color = if is_selected { ec.selected_border_color.to_iced() } else { ec.border_color.to_iced() };
    let r            = ec.border_radius;

    button::Style
    {
        background: Some(iced::Background::Color(bg)),
        text_color: ec.text_color.to_iced(),
        border: Border
        {
            color:  border_color,
            width:  ec.border_width,
            radius: Radius { top_left: r[0], top_right: r[1], bottom_left: r[2], bottom_right: r[3] },
        },
        shadow: Shadow
        {
            color:       ec.shadow_color.to_iced(),
            offset:      Vector::new(ec.shadow_offset_x, ec.shadow_offset_y),
            blur_radius: ec.shadow_blur,
        },
        snap: false,
    }
}
