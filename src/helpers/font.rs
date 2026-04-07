// ============ IMPORTS ============
use std::collections::HashSet;
use strsim::levenshtein;
use iced::font::Family;
use iced::Font;

use crate::ron::{FontStyle, FontWeight};
use crate::helpers::widget::intern_font_family;




// ============ CONST ============
pub const MIN_FUZZY_LEN: usize = 4;
pub const MAX_FUZZY_RATIO: f32 = 0.3;




// ============ FUNCTIONS ============
pub fn build_font(family: &str, weight: &FontWeight, style: &FontStyle) -> Font
{
	if family.is_empty()
	{
		return Font
		{
			weight: weight.to_iced(),
			style:  style.to_iced(),
			..Font::DEFAULT
		};
	}

	match resolve_font(family)
	{
		Some(resolved) =>
		{
			if resolved != family
			{
				eprintln!("[icelauncher] font '{}' resolved to '{}'", family, resolved);
			}
			Font
			{
				family: Family::Name(intern_font_family(&resolved)),
				weight: weight.to_iced(),
				style:  style.to_iced(),
				..Font::DEFAULT
			}
		}
		None =>
		{
			eprintln!("[icelauncher] font '{}' not found — using system default", family);
			Font
			{
				weight: weight.to_iced(),
				style:  style.to_iced(),
				..Font::DEFAULT
			}
		}
	}
}



pub fn resolve_font(requested: &str) -> Option<String>
{
	let normalized_requested = normalize(requested);
	let fonts = system_fonts();

	// 1. Exact normalized match
	for font in &fonts
	{
		if normalize(font) == normalized_requested
		{
			return Some(font.clone());
		}
	}

	// 2. Normalized system name starts with the request
	for font in &fonts
	{
		if normalize(font).starts_with(&normalized_requested)
		{
			return Some(font.clone());
		}
	}

	// 3. Request starts with a normalized system name (reverse prefix)
	for font in &fonts
	{
		let nf = normalize(font);
		if nf.len() >= MIN_FUZZY_LEN && normalized_requested.starts_with(&nf)
		{
			return Some(font.clone());
		}
	}

	// 4. Substring containment (both directions), only for longer queries
	if normalized_requested.len() >= MIN_FUZZY_LEN
	{
		for font in &fonts
		{
			let nf = normalize(font);
			if nf.contains(&normalized_requested) || normalized_requested.contains(&nf)
			{
				return Some(font.clone());
			}
		}
	}

	// 5. Fuzzy Levenshtein distance
	if normalized_requested.len() >= MIN_FUZZY_LEN
	{
		let max_allowed = ((normalized_requested.len() as f32 * MAX_FUZZY_RATIO).floor() as usize).max(1);
		let mut best_font = None;
		let mut best_distance = usize::MAX;

		for font in &fonts
		{
			let nf = normalize(font);
			let candidate = if nf.len() > normalized_requested.len()
			{
				let char_count = normalized_requested.chars().count();
				let byte_end   = nf.char_indices().nth(char_count).map(|(i, _)| i).unwrap_or(nf.len());
				&nf[..byte_end]
			}
			else
			{
				&nf
			};

			let dist = levenshtein(&normalized_requested, candidate);
			if dist < best_distance
			{
				best_distance = dist;
				best_font = Some(font.clone());
			}
		}

		if best_distance <= max_allowed
			&& let Some(font) = best_font
			{
				return Some(font);
			}
	}

	None
}



fn system_fonts() -> Vec<String>
{
	let output = std::process::Command::new("fc-list").args(["-f", "%{family}\n"]).output();

	if let Ok(out) = output
	{
		let mut set = HashSet::new();
		for line in String::from_utf8_lossy(&out.stdout).lines()
		{
			for fam in line.split(',')
			{
				set.insert(fam.trim().to_string());
			}
		}
		let mut fonts: Vec<String> = set.into_iter().collect();
		fonts.sort_by_key(|f| f.len());
		fonts
	}
	else
	{
		Vec::new()
	}
}



pub fn normalize(name: &str) -> String
{
	let lower = name.to_lowercase();
	lower
		.split([' ', '-', '_'])
		.filter(|word| !matches!(*word, "nerd" | "font" | "fonts" | "nf" | "nfm" | "nfp"))
		.collect::<String>()
}
