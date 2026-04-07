// ============ IMPORTS ============
use iced::Font;
use iced::Length;
use iced::border::Radius;
use std::collections::HashMap;
use std::sync::Mutex;




// ============ CRATES ============
use crate::ron::{FontStyle, FontWeight, TextAlign};




// ============ STATIC'S/CONST'S ============
static FONT_FAMILY_CACHE: Mutex<Option<HashMap<String, &'static str>>> = Mutex::new(None);




// ============ FUNCTIONS ============
pub fn intern_font_family(family: &str) -> &'static str
{
	let mut guard = FONT_FAMILY_CACHE.lock().unwrap_or_else(|e| e.into_inner());
	let cache = guard.get_or_insert_with(HashMap::new);
	if let Some(&s) = cache.get(family) 
        {
		return s;
	}
	let leaked: &'static str = Box::leak(family.to_string().into_boxed_str());
	cache.insert(family.to_string(), leaked);
	leaked
}



pub fn make_font(weight: &FontWeight, style: &FontStyle) -> Font
{
	Font { weight: weight.to_iced(), style: style.to_iced(), ..Font::default() }
}


pub fn make_font_family(weight: &FontWeight, style: &FontStyle, family: &str) -> Font
{
	crate::helpers::font::build_font(family, weight, style)
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
