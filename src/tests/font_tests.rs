// ============ TESTS ============
#[cfg(test)]
mod tests
{
        use strsim::levenshtein;
        use crate::helpers::font::{MIN_FUZZY_LEN, MAX_FUZZY_RATIO, normalize};

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
