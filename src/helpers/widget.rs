// ============ IMPORTS ============
use iced::Font;
use iced::Length;
use iced::border::Radius;




// ============ CRATES ============
use crate::ron::{FontStyle, FontWeight, TextAlign};




// ============ FUNCTIONS ============
pub fn make_font(weight: &FontWeight, style: &FontStyle) -> Font
{
	Font { weight: weight.to_iced(), style: style.to_iced(), ..Font::default() }
}


pub fn horizontal_align(align: &TextAlign) -> iced::alignment::Horizontal
{
	align.to_iced()
}


pub fn corner_radius(r: [f32; 4]) -> Radius
{
	Radius { top_left: r[0], top_right: r[1], bottom_left: r[2], bottom_right: r[3] }
}


pub fn optional_length(px: u32) -> Length
{
	if px > 0 { Length::Fixed(px as f32) } else { Length::Fill }
}


pub fn optional_length_shrink(px: u32) -> Length
{
	if px > 0 { Length::Fixed(px as f32) } else { Length::Shrink }
}
