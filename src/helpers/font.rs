// ============ IMPORTS ============
use std::collections::HashSet;
use strsim::levenshtein;
use iced::font::Family;
use iced::Font;

use crate::ron::{FontStyle, FontWeight};
use crate::helpers::widget::intern_font_family;




// ============ CONST ============
const MIN_FUZZY_LEN: usize = 4;
const MAX_FUZZY_RATIO: f32 = 0.3;




// ============ FUNCTIONS ============

/// Build an iced `Font` from a family name string, running it through the
/// fuzzy resolver so that user-friendly names like "JetBrains Mono" or
/// "FiraCode" work even if the exact fc-list name differs.
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


/// Try to match `requested` against the system font list, using a cascade of
/// increasingly lenient strategies (exact → prefix → reverse-prefix →
/// substring → fuzzy Levenshtein).  Returns the canonical fc-list name on
/// success, or `None` if nothing close enough is found.
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
			// Compare only the prefix of the longer string so we don't
			// penalise extra style words like "Nerd Font ExtraBold".
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


/// List every font family available on the system via `fc-list`.
/// Results are sorted shortest-first so base family names beat variant names
/// during prefix matching (e.g. "JetBrainsMono Nerd Font" beats
/// "JetBrainsMono NFM ExtraBold").
fn system_fonts() -> Vec<String>
{
	let output = std::process::Command::new("fc-list")
		.args(["-f", "%{family}\n"])
		.output();

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


/// Strip noise words and separators, then lowercase, so that
/// "JetBrains Mono Nerd Font" and "JetBrainsMono NF" both normalize to
/// "jetbrainsmono".
fn normalize(name: &str) -> String
{
	let lower = name.to_lowercase();
	lower
		.split([' ', '-', '_'])
		.filter(|word| !matches!(*word, "nerd" | "font" | "fonts" | "nf" | "nfm" | "nfp"))
		.collect::<String>()
}




// ============ TESTS ============
#[cfg(test)]
mod tests
{
	use super::*;

	fn resolve_from(requested: &str, fonts: &[&str]) -> Option<String>
	{
		let normalized_requested = normalize(requested);
		let fonts: Vec<String> = fonts.iter().map(|s| s.to_string()).collect();

		for font in &fonts { if normalize(font) == normalized_requested { return Some(font.clone()); } }
		for font in &fonts { if normalize(font).starts_with(&normalized_requested) { return Some(font.clone()); } }
		for font in &fonts
		{
			let nf = normalize(font);
			if nf.len() >= MIN_FUZZY_LEN && normalized_requested.starts_with(&nf) { return Some(font.clone()); }
		}
		if normalized_requested.len() >= MIN_FUZZY_LEN
		{
			for font in &fonts
			{
				let nf = normalize(font);
				if nf.contains(&normalized_requested) || normalized_requested.contains(&nf) { return Some(font.clone()); }
			}
		}
		if normalized_requested.len() >= MIN_FUZZY_LEN
		{
			let max_allowed = ((normalized_requested.len() as f32 * MAX_FUZZY_RATIO).floor() as usize).max(1);
			let mut best_font = None;
			let mut best_distance = usize::MAX;
			for font in &fonts
			{
				let nf = normalize(font);
				let candidate = if nf.len() > normalized_requested.len() { nf[..normalized_requested.len()].to_string() } else { nf };
				let dist = levenshtein(&normalized_requested, &candidate);
				if dist < best_distance { best_distance = dist; best_font = Some(font.clone()); }
			}
			if best_distance <= max_allowed { if let Some(font) = best_font { return Some(font); } }
		}
		None
	}

	const TEST_FONTS: &[&str] = &[
		"JetBrainsMono Nerd Font",
		"JetBrainsMono NF",
		"JetBrainsMono NFM",
		"JetBrainsMonoNL Nerd Font",
		"FiraCode Nerd Font",
		"FiraCode NF",
		"Ubuntu",
		"DejaVu Sans",
	];

	#[test] fn normalize_strips_noise_words()       { assert_eq!(normalize("JetBrainsMono Nerd Font"), "jetbrainsmono"); }
	#[test] fn normalize_strips_separators()        { assert_eq!(normalize("Fira-Code"), "firacode"); }
	#[test] fn normalize_lowercases()               { assert_eq!(normalize("JetBrainsMono"), "jetbrainsmono"); }
	#[test] fn normalize_empty()                    { assert_eq!(normalize(""), ""); }
	#[test] fn normalize_only_noise()               { assert_eq!(normalize("Nerd Font"), ""); }

	#[test] fn exact_name_matches()                 { assert_eq!(resolve_from("JetBrains Mono", TEST_FONTS), Some("JetBrainsMono Nerd Font".into())); }
	#[test] fn name_without_spaces_matches()        { assert_eq!(resolve_from("JetBrainsMono", TEST_FONTS), Some("JetBrainsMono Nerd Font".into())); }
	#[test] fn fira_code_matches()                  { assert_eq!(resolve_from("Fira Code", TEST_FONTS), Some("FiraCode Nerd Font".into())); }
	#[test] fn ubuntu_exact_match()                 { assert_eq!(resolve_from("Ubuntu", TEST_FONTS), Some("Ubuntu".into())); }
	#[test] fn unknown_font_returns_none()          { assert_eq!(resolve_from("Nonexistent XYZ", TEST_FONTS), None); }
	#[test] fn short_input_returns_none()           { assert_eq!(resolve_from("ab", TEST_FONTS), None); }
	#[test] fn does_not_match_wrong_font()
	{
		let r = resolve_from("JetBrains Mono", TEST_FONTS);
		assert!(r.as_deref().map(|s| !s.to_lowercase().contains("fira")).unwrap_or(true));
	}
}
