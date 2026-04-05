// ============ IMPORTS ============
use std::path::{Path, PathBuf};




// ============ CRATES ============
use crate::{AppEntry, Message};




// ============ FUNCTIONS ============
pub fn load_apps_stream() -> impl futures::Stream<Item = Message>
{
    async_stream::stream!
    {
        let entries = tokio::task::spawn_blocking(scan_desktop_files).await.unwrap_or_else(|e| { eprintln!("[icelauncher] Failed to scan desktop files: {e}"); vec![] });
        yield Message::EntriesLoaded(entries);
    }
}



pub fn scan_desktop_files() -> Vec<AppEntry>
{
    let search_dirs: Vec<PathBuf> =
    {
        let mut dirs = Vec::new();
        if let Some(home) = home::home_dir() { dirs.push(home.join(".local/share/applications")); }
        if let Ok(xdg_dirs) = std::env::var("XDG_DATA_DIRS") { for p in xdg_dirs.split(':') { dirs.push(PathBuf::from(p).join("applications")); } }
        dirs.push(PathBuf::from("/usr/local/share/applications")); 
        dirs.push(PathBuf::from("/usr/share/applications")); 
        dirs
    };

    let icon_bases  = crate::helpers::icon::icon_base_dirs();
    let icon_themes = crate::helpers::icon::discover_themes(&icon_bases);

    let mut entries:    Vec<AppEntry>                     = Vec::new();
    let mut seen_names: std::collections::HashSet<String> = std::collections::HashSet::new();

    for dir in search_dirs
    {
        let Ok(read) = std::fs::read_dir(&dir) else { continue; };
        for file in read.flatten()
        {
            let path = file.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") { continue; }
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            if let Some(mut entry) = parse_desktop_file(&path) && seen_names.insert(stem)
            {
                entry.icon_path = crate::helpers::icon::resolve_icon_with(&entry.icon, &icon_bases, &icon_themes);
                entries.push(entry);
            }
        }
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries
}


pub fn parse_desktop_file(path: &Path) -> Option<AppEntry>
{
    let content = std::fs::read_to_string(path).ok()?;

    let mut name:     String      = String::new();
    let mut exec:     String      = String::new();
    let mut comment:  String      = String::new();
    let mut icon:     String      = String::new();
    let mut keywords: Vec<String> = Vec::new();
    let mut terminal   = false;
    let mut no_display = false;
    let mut in_section = false;

    for line in content.lines()
    {
        let line = line.trim();
        if line == "[Desktop Entry]"                               { in_section = true;  continue; }
        if line.starts_with('[')                                   { in_section = false; continue; }
        if !in_section                                             { continue; }

        if let Some(v) = line.strip_prefix("Name=")     && name.is_empty()    { name    = v.to_string(); }
        if let Some(v) = line.strip_prefix("Exec=")     && exec.is_empty()    { exec    = sanitize_exec(v); }
        if let Some(v) = line.strip_prefix("Comment=")  && comment.is_empty() { comment = v.to_string(); }
        if let Some(v) = line.strip_prefix("Icon=")     && icon.is_empty()    { icon    = v.to_string(); }
        if let Some(v) = line.strip_prefix("Keywords=") && keywords.is_empty()
        {
            keywords = v.split(';').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect();
        }
        if line == "Terminal=true"  { terminal   = true; }
        if line == "NoDisplay=true" { no_display = true; }
        if line.starts_with("Type=") && !line.contains("Application") { return None; }
    }

    if no_display || name.is_empty() || exec.is_empty() { return None; }
    Some(AppEntry { name, exec, comment, icon, icon_path: None, keywords, terminal })
}



pub fn tokenize(s: &str) -> Vec<String>
{
    let mut tokens:     Vec<String> = Vec::new();
    let mut current                 = String::new();
    let mut in_quotes               = false;
    let mut quote_char              = ' ';

    for ch in s.chars()
    {
        match ch
        {
            '"' | '\'' if !in_quotes =>
            {
                in_quotes  = true;
                quote_char = ch;
            }
            c if in_quotes && c == quote_char =>
            {
                in_quotes = false;
            }
            ' ' if !in_quotes =>
            {
                if !current.is_empty()
                {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() { tokens.push(current); }
    tokens
}



pub fn sanitize_exec(exec: &str) -> String
{
    tokenize(exec).into_iter().filter(|tok| !tok.starts_with('%')).collect::<Vec<_>>().join(" ")
}
