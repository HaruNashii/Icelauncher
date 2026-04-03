// ============ FUNCTIONS ============
pub fn truncate(s: &str, max: usize) -> String
{
    if max == 0 || s.chars().count() <= max { return s.to_string(); }
    if max <= 1 { return "…".to_string(); }
    let cut: String = s.chars().take(max.saturating_sub(1)).collect();
    format!("{}…", cut)
}
