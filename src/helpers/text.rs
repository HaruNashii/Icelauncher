// ============ FUNCTIONS ============
pub fn truncate_with(s: &str, max: usize, ellipsis: &str) -> String
{
	if max == 0 || s.chars().count() <= max
	{
		return s.to_string();
	}

	let keep = max.saturating_sub(ellipsis.chars().count());
	let truncated: String = s.chars().take(keep).collect();
	format!("{}{}", truncated, ellipsis)
}


pub fn apply_entry_text_rules(s: &str, max_chars: usize, ellipsize: bool, _wrap: bool, ellipsis: &str) -> String
{
	if ellipsize && max_chars > 0 { truncate_with(s, max_chars, ellipsis) } else { s.to_string() }
}
