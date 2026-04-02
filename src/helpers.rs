// ============ IMPORTS ============
use iced::{Vector, Color, Theme, border::Radius, widget::button, widget::button::Status};
use iced_layershell::reexport::core::{Shadow, Border};
use std::{path::PathBuf, process::Command as StdCommand};

use crate::{AppData, AppEntry, Message};
use crate::ron::LauncherConfig;




// ============ APP LOADING ============
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

    let mut entries: Vec<AppEntry> = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    for dir in search_dirs
    {
        let Ok(read) = std::fs::read_dir(&dir) else { continue; };
        for file in read.flatten()
        {
            let path = file.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") { continue; }
            if let Some(entry) = parse_desktop_file(&path) && seen_names.insert(entry.name.clone())
            {
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
    let mut name     = String::new();
    let mut exec     = String::new();
    let mut comment  = String::new();
    let mut icon     = String::new();
    let mut keywords: Vec<String> = Vec::new();
    let mut terminal     = false;
    let mut no_display   = false;
    let mut in_section   = false;

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
        if line == "Terminal=true"    { terminal   = true; }
        if line == "NoDisplay=true"   { no_display = true; }
        if line.starts_with("Type=") && !line.contains("Application") { return None; }
    }

    if no_display || name.is_empty() || exec.is_empty() { return None; }
    Some(AppEntry { name, exec, comment, icon, keywords, terminal })
}



pub fn sanitize_exec(exec: &str) -> String
{
    exec.split_whitespace()
        .filter(|tok| !tok.starts_with('%'))
        .collect::<Vec<_>>()
        .join(" ")
}



pub fn launch_app(exec: &str, cfg: &LauncherConfig, terminal: bool)
{
    let full_cmd = if terminal && !cfg.behaviour.terminal_command.is_empty()
    {
        format!("{} {}", cfg.behaviour.terminal_command.trim(), exec)
    }
    else
    {
        exec.to_string()
    };

    let parts: Vec<&str> = full_cmd.split_whitespace().collect();
    if let Some((prog, args)) = parts.split_first()
    {
        let _ = StdCommand::new(prog).args(args).spawn();
    }
}




// ============ SEARCH ============
pub fn filter_entries(entries: &[AppEntry], query: &str, cfg: &LauncherConfig) -> Vec<AppEntry>
{
    if query.is_empty() { return entries.to_vec(); }

    let q = if cfg.behaviour.case_sensitive { query.to_string() } else { query.to_lowercase() };

    let normalize = |s: &str| -> String
    {
        if cfg.behaviour.case_sensitive { s.to_string() } else { s.to_lowercase() }
    };

    let mut scored: Vec<(usize, AppEntry)> = entries.iter().filter_map(|e|
    {
        let name     = normalize(&e.name);
        let comment  = normalize(&e.comment);
        let exec     = normalize(&e.exec);

        let name_match    = cfg.behaviour.search_name    && name.contains(&q);
        let comment_match = cfg.behaviour.search_comment && comment.contains(&q);
        let exec_match    = cfg.behaviour.search_exec    && exec.contains(&q);
        let kw_match      = cfg.behaviour.search_keywords && e.keywords.iter().any(|k| normalize(k).contains(&q));

        if !(name_match || comment_match || exec_match || kw_match) { return None; }

        // Score: lower = better
        let score = if name.starts_with(&q)   { 0 }
                    else if name_match         { 1 }
                    else if kw_match           { 2 }
                    else if exec_match         { 3 }
                    else                       { 4 };

        Some((score, e.clone()))
    }).collect();

    scored.sort_by(|(sa, a), (sb, b)| sa.cmp(sb).then(a.name.to_lowercase().cmp(&b.name.to_lowercase())));
    scored.into_iter().map(|(_, e)| e).collect()
}




// ============ STYLE HELPERS ============
pub fn global_style(_app: &AppData, _theme: &Theme) -> iced_layershell::reexport::core::theme::Style
{
    iced_layershell::reexport::core::theme::Style
    {
        background_color: Color::TRANSPARENT,
        text_color:       Color::WHITE,
    }
}



pub fn entry_button_style(status: Status, is_selected: bool, cfg: &LauncherConfig) -> button::Style
{
    let ec = &cfg.entry;

    let bg = match (is_selected, status)
    {
        (_, Status::Pressed)      => ec.pressed_color.to_iced(),
        (true, Status::Hovered)   => ec.selected_hovered_color.to_iced(),
        (true, _)                 => ec.selected_color.to_iced(),
        (false, Status::Hovered)  => ec.hovered_color.to_iced(),
        (false, _)                => ec.background_color.to_iced(),
    };

    let border_color = if is_selected { ec.selected_border_color.to_iced() } else { ec.border_color.to_iced() };
    let r            = ec.border_radius;

    button::Style
    {
        background: Some(iced::Background::Color(bg)),
        text_color: ec.text_color.to_iced(),
        border: Border
        {
            color:  border_color,
            width:  ec.border_width,
            radius: Radius { top_left: r[0], top_right: r[1], bottom_left: r[2], bottom_right: r[3] },
        },
        shadow: Shadow
        {
            color:       ec.shadow_color.to_iced(),
            offset:      Vector::new(ec.shadow_offset_x, ec.shadow_offset_y),
            blur_radius: ec.shadow_blur,
        },
        snap: false,
    }
}



pub fn derive_icon_char(name: &str) -> &'static str
{
    let lower = name.to_lowercase();
    if lower.contains("terminal")  || lower.contains("alacritty") || lower.contains("kitty")     || lower.contains("foot")     { return "⊞"; }
    if lower.contains("firefox")   || lower.contains("chromium")  || lower.contains("browser")   || lower.contains("chrome")   { return "🌐"; }
    if lower.contains("music")     || lower.contains("spotify")   || lower.contains("rhythmbox")  || lower.contains("mpv")     { return "♫"; }
    if lower.contains("file")      || lower.contains("nautilus")  || lower.contains("dolphin")   || lower.contains("thunar")   { return "📁"; }
    if lower.contains("text")      || lower.contains("gedit")     || lower.contains("notepad")   || lower.contains("kate")     { return "📝"; }
    if lower.contains("image")     || lower.contains("photo")     || lower.contains("gimp")      || lower.contains("inkscape") { return "🖼"; }
    if lower.contains("code")      || lower.contains("vscode")    || lower.contains("vscodium")                                { return "</>"; }
    if lower.contains("setting")   || lower.contains("control")   || lower.contains("system")                                  { return "⚙"; }
    if lower.contains("mail")      || lower.contains("thunderbird")|| lower.contains("email")                                  { return "✉"; }
    if lower.contains("calc")      || lower.contains("math")                                                                   { return "🧮"; }
    "▶"
}



pub fn truncate(s: &str, max: usize) -> String
{
    if max == 0 || s.chars().count() <= max { return s.to_string(); }
    let cut: String = s.chars().take(max.saturating_sub(1)).collect();
    format!("{}…", cut)
}
