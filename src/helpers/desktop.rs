// ============ IMPORTS ============
use std::path::PathBuf;




// ============ CRATES ============
use crate::{AppEntry, Message};
use crate::helpers::icon::resolve_icon;




// ============ FUNCTIONS ============
pub fn load_apps_stream() -> impl futures::Stream<Item = Message>
{
    async_stream::stream!
    {
        let entries = tokio::task::spawn_blocking(scan_desktop_files).await.unwrap_or_default();
        yield Message::EntriesLoaded(entries);
    }
}



pub fn scan_desktop_files() -> Vec<AppEntry>
{
    let search_dirs: Vec<PathBuf> =
    {
        let mut dirs = vec!
        [
            PathBuf::from("/usr/share/applications"),
            PathBuf::from("/usr/local/share/applications"),
        ];
        if let Some(home) = home::home_dir() { dirs.push(home.join(".local/share/applications")); }
        if let Ok(xdg_dirs) = std::env::var("XDG_DATA_DIRS")
        {
            for p in xdg_dirs.split(':') { dirs.push(PathBuf::from(p).join("applications")); }
        }
        dirs
    };

    let mut entries:    Vec<AppEntry>                     = Vec::new();
    let mut seen_names: std::collections::HashSet<String> = std::collections::HashSet::new();

    for dir in search_dirs
    {
        let Ok(read) = std::fs::read_dir(&dir) else { continue; };
        for file in read.flatten()
        {
            let path = file.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") { continue; }
            if let Some(mut entry) = parse_desktop_file(&path) && seen_names.insert(entry.name.clone())
            {
                entry.icon_path = resolve_icon(&entry.icon);
                entries.push(entry);
            }
        }
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries
}



pub fn parse_desktop_file(path: &PathBuf) -> Option<AppEntry>
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



pub fn sanitize_exec(exec: &str) -> String
{
    exec.split_whitespace().filter(|tok| !tok.starts_with('%')).collect::<Vec<_>>().join(" ")
}
