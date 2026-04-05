// ============ IMPORTS ============
use std::path::{Path, PathBuf};

use rayon::prelude::*;




// ============ CRATES ============
use crate::{AppEntry, Message};




// ============ FUNCTIONS ============
pub fn load_apps_stream() -> impl futures::Stream<Item = Message>
{
	async_stream::stream! {
		let entries = tokio::task::spawn_blocking(scan_desktop_files)
			.await
			.unwrap_or_else(|e| {
				eprintln!("[icelauncher] Failed to scan desktop files: {e}");
				vec![]
			});
		yield Message::EntriesLoaded(entries);
	}
}


pub fn scan_desktop_files() -> Vec<AppEntry>
{
	let search_dirs = application_search_dirs();
	let icon_bases = crate::helpers::icon::icon_base_dirs();
	let icon_themes = crate::helpers::icon::discover_themes(&icon_bases);

	let mut seen_stems = std::collections::HashSet::new();
	let mut paths: Vec<PathBuf> = Vec::new();

	for dir in search_dirs {
		let Ok(dir_entries) = std::fs::read_dir(&dir) else { continue };
		for file in dir_entries.flatten() {
			let path = file.path();
			if !is_desktop_file(&path) {
				continue;
			}
			let stem = file_stem(&path);
			if seen_stems.insert(stem) {
				paths.push(path);
			}
		}
	}

	let mut entries: Vec<AppEntry> = paths
		.par_iter()
		.filter_map(|path| {
			let mut entry = parse_desktop_file(path)?;
			entry.icon_path =
				crate::helpers::icon::resolve_icon_with(&entry.icon, &icon_bases, &icon_themes);
			Some(entry.with_normalized())
		})
		.collect();

	entries.sort_unstable_by(|a, b| a.name_lc.cmp(&b.name_lc));
	entries
}


pub fn parse_desktop_file(path: &Path) -> Option<AppEntry>
{
	let bytes = std::fs::read(path).ok()?;
	let content = String::from_utf8_lossy(&bytes);

	let mut name = String::new();
	let mut exec = String::new();
	let mut comment = String::new();
	let mut icon = String::new();
	let mut keywords = Vec::new();
	let mut terminal = false;
	let mut no_display = false;
	let mut in_desktop_entry = false;

	for raw_line in content.lines() {
		let line = raw_line.trim();

		if line == "[Desktop Entry]" {
			in_desktop_entry = true;
			continue;
		}
		if line.starts_with('[') {
			in_desktop_entry = false;
			continue;
		}
		if !in_desktop_entry {
			continue;
		}

		if line.starts_with("Type=") && !line.contains("Application") {
			return None;
		}
		if line == "NoDisplay=true" {
			no_display = true;
		}
		if line == "Terminal=true" {
			terminal = true;
		}

		if let Some(v) = line.strip_prefix("Name=")
			&& name.is_empty()
		{
			name = v.to_string();
		}
		if let Some(v) = line.strip_prefix("Exec=")
			&& exec.is_empty()
		{
			exec = sanitize_exec(v);
		}
		if let Some(v) = line.strip_prefix("Comment=")
			&& comment.is_empty()
		{
			comment = v.to_string();
		}
		if let Some(v) = line.strip_prefix("Icon=")
			&& icon.is_empty()
		{
			icon = v.to_string();
		}
		if let Some(v) = line.strip_prefix("Keywords=")
			&& keywords.is_empty()
		{
			keywords = parse_keywords(v);
		}
	}

	if no_display || name.is_empty() || exec.is_empty() {
		return None;
	}

	Some(AppEntry {
		name,
		exec,
		comment,
		icon,
		icon_path: None,
		keywords,
		terminal,
		name_lc: String::new(),
		exec_lc: String::new(),
		comment_lc: String::new(),
		keywords_lc: Vec::new(),
	})
}


pub fn tokenize(s: &str) -> Vec<String>
{
	let mut tokens = Vec::new();
	let mut current = String::new();
	let mut in_quotes = false;
	let mut quote_char = ' ';

	for ch in s.chars() {
		match ch
		{
			'"' | '\'' if !in_quotes => {
				in_quotes = true;
				quote_char = ch;
			}
			c if in_quotes && c == quote_char => {
				in_quotes = false;
			}
			' ' if !in_quotes => {
				if !current.is_empty() {
					tokens.push(current.clone());
					current.clear();
				}
			}
			_ => current.push(ch),
		}
	}

	if !current.is_empty() {
		tokens.push(current);
	}
	tokens
}


pub fn sanitize_exec(exec: &str) -> String
{
	tokenize(exec).into_iter().filter(|token| !token.starts_with('%')).collect::<Vec<_>>().join(" ")
}


fn application_search_dirs() -> Vec<PathBuf>
{
	let mut dirs = Vec::new();

	if let Some(home) = home::home_dir() {
		dirs.push(home.join(".local/share/applications"));
	}

	if let Ok(xdg_dirs) = std::env::var("XDG_DATA_DIRS") {
		for path in xdg_dirs.split(':') {
			dirs.push(PathBuf::from(path).join("applications"));
		}
	}

	dirs.push(PathBuf::from("/usr/local/share/applications"));
	dirs.push(PathBuf::from("/usr/share/applications"));
	dirs
}


fn is_desktop_file(path: &Path) -> bool
{
	path.extension().and_then(|e| e.to_str()) == Some("desktop")
}


fn file_stem(path: &Path) -> String
{
	path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string()
}


fn parse_keywords(raw: &str) -> Vec<String>
{
	raw.split(';').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect()
}
