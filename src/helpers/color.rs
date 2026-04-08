// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use crate::ron::EntryConfig;
use iced::Color;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct ConvertedEntriesColor
{
    pub name_color: Color,
    pub name_hovered_color: Color,
    pub name_selected_color: Color,
    pub comment_color: Color,
    pub comment_hovered_color: Color,
    pub comment_selected_color: Color,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Gradient
{
	Gradient((f32, Vec<(f32, ColorType)>)),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum ColorType
{
	RGB([u32; 3]),
	RGBA([u32; 4]),
	HEX([u8; 9]),
}




// ============ IMPL'S ============
impl Default for ColorType
{
	fn default() -> Self
	{
		ColorType::RGB([255, 255, 255])
	}
}

impl<'de> Deserialize<'de> for ColorType
{
	fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error>
	{
		#[derive(Deserialize)]
		#[allow(clippy::upper_case_acronyms)]
		enum Helper
		{
			RGB([u32; 3]),
			RGBA([u32; 4]),
			HEX(String),
		}
		match Helper::deserialize(d)?
		{
			Helper::RGB(v) => Ok(ColorType::RGB(v)),
			Helper::RGBA(v) => Ok(ColorType::RGBA(v)),
			Helper::HEX(s) => Ok(hex_color(&s)),
		}
	}
}

impl ColorType
{
	pub fn to_iced(self) -> iced::Color
	{
		match self
		{
			ColorType::RGB([r, g, b]) => iced::Color::from_rgb8(r as u8, g as u8, b as u8),
			ColorType::RGBA([r, g, b, a]) => iced::Color::from_rgba8(
				r as u8,
				g as u8,
				b as u8,
				(a as f32).clamp(0., 100.) / 100.,
			),
			ColorType::HEX(bytes) => hex_to_iced(&bytes).unwrap_or(iced::Color::WHITE),
		}
	}
}




// ============ FUNCTIONS ============
pub fn convert_entry_color(entry_config: &EntryConfig) -> ConvertedEntriesColor
{
    ConvertedEntriesColor
    {
        name_color: entry_config.name_color.to_iced(),
        name_hovered_color: entry_config.hovered_name_color.to_iced(),
        name_selected_color: entry_config.selected_name_color.to_iced(),

        comment_color: entry_config.comment_color.to_iced(),
        comment_hovered_color: entry_config.hovered_comment_color.to_iced(),
        comment_selected_color: entry_config.selected_comment_color.to_iced()
    }
}

pub fn hex_color(s: &str) -> ColorType
{
	let mut bytes = [0u8; 9];
	let src = s.trim_start_matches('#').as_bytes();
	let len = src.len().min(9);
	bytes[..len].copy_from_slice(&src[..len]);
	ColorType::HEX(bytes)
}


pub fn hex_to_iced(bytes: &[u8; 9]) -> Option<iced::Color>
{
	let end = bytes.iter().position(|&b| b == 0).unwrap_or(9);
	let s = std::str::from_utf8(&bytes[..end]).ok()?;
	let hex = s.trim_start_matches('#');
	if hex.len() == 6 {
		let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
		let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
		let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
		Some(iced::Color::from_rgb8(r, g, b))
	} else if hex.len() == 8 {
		let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
		let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
		let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
		let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
		Some(iced::Color::from_rgba8(r, g, b, a as f32 / 255.))
	} else {
		None
	}
}


pub fn color_or_gradient(gradient: Option<&Gradient>, color: ColorType) -> iced::Background
{
	use iced_layershell::reexport::core::{Degrees, gradient::Linear};
	match gradient
	{
		Some(Gradient::Gradient((angle, stops))) =>
		{
			let mut g = Linear::new(Degrees(*angle));
			for (pos, col) in stops
			{
				g = g.add_stop(*pos, col.to_iced());
			}
			iced::Background::Gradient(g.into())
		}
		None => iced::Background::Color(color.to_iced()),
	}
}
