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


pub fn load_shell_commands_stream() -> impl futures::Stream<Item = Message>
{
	async_stream::stream! {
		let entries = tokio::task::spawn_blocking(scan_shell_commands)
			.await
			.unwrap_or_else(|e| {
				eprintln!("[icelauncher] Failed to scan shell commands: {e}");
				vec![]
			});
		yield Message::EntriesLoaded(entries);
	}
}


/// Collect all executables reachable via $PATH and return them as AppEntry items.
pub fn scan_shell_commands() -> Vec<AppEntry>
{
	let path_var = std::env::var("PATH").unwrap_or_default();
	let mut seen = std::collections::HashSet::new();
	let mut entries: Vec<AppEntry> = Vec::new();

	for dir in path_var.split(':').filter(|d| !d.is_empty())
	{
		let Ok(dir_entries) = std::fs::read_dir(dir) else { continue };
		for file in dir_entries.flatten()
		{
			let path = file.path();
			// Must be executable and not a directory.
			if !is_executable(&path) { continue; }
			let Some(name) = path.file_name().and_then(|n| n.to_str()) else { continue };
			if !seen.insert(name.to_string()) { continue; }

			let entry = AppEntry {
				name: name.to_string(),
				generic_name: String::new(),
				exec: name.to_string(),
				comment: String::new(),
				icon: String::new(),
				icon_path: None,
				keywords: Vec::new(),
				terminal: false,
				name_lc: String::new(),
				generic_name_lc: String::new(),
				exec_lc: String::new(),
				comment_lc: String::new(),
				keywords_lc: Vec::new(),
			};
			entries.push(entry.with_normalized());
		}
	}

	entries.sort_unstable_by(|a, b| a.name_lc.cmp(&b.name_lc));
	entries
}


fn is_executable(path: &Path) -> bool
{
	use std::os::unix::fs::PermissionsExt;
	let Ok(meta) = path.metadata() else { return false };
	if meta.is_dir() { return false; }
	meta.permissions().mode() & 0o111 != 0
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
	parse_desktop_file_for_session(path, current_desktop_names().as_deref())
}


pub fn parse_desktop_file_for_session(path: &Path, current_desktop: Option<&[String]>) -> Option<AppEntry>
{
	let bytes = std::fs::read(path).ok()?;
	let content = String::from_utf8_lossy(&bytes);

	// Detect the user's locale (e.g. "en_US", "fr_FR") for Name[xx] parsing.
	let locale = std::env::var("LANG")
		.unwrap_or_default()
		.split('.')
		.next()
		.unwrap_or("")
		.to_string();
	// Also build a short form like "en" from "en_US".
	let locale_short: String = locale.split('_').next().unwrap_or("").to_string();

	let mut name = String::new();
	let mut name_locale = String::new();           // Name[xx_YY]=
	let mut name_locale_short = String::new();     // Name[xx]=
	let mut generic_name = String::new();
	let mut generic_locale = String::new();        // GenericName[xx_YY]=
	let mut generic_locale_short = String::new();  // GenericName[xx]=
	let mut exec = String::new();
	let mut try_exec = String::new();
	let mut comment = String::new();
	let mut comment_locale = String::new();        // Comment[xx_YY]=
	let mut comment_locale_short = String::new();  // Comment[xx]=
	let mut icon = String::new();
	let mut keywords = Vec::new();
	let mut terminal = false;
	let mut no_display = false;
	let mut hidden = false;
	let mut only_show_in: Option<Vec<String>> = None;
	let mut not_show_in: Vec<String> = Vec::new();
	let mut in_desktop_entry = false;

	for raw_line in content.lines()
	{
		let line = raw_line.trim();

		if line == "[Desktop Entry]"
		{
			in_desktop_entry = true;
			continue;
		}
		if line.starts_with('[')
		{
			in_desktop_entry = false;
			continue;
		}
		if !in_desktop_entry
		{
			continue;
		}

		if line.starts_with("Type=") && !line.contains("Application")
		{
			return None;
		}
		if line == "NoDisplay=true"  { no_display = true; }
		if line == "Hidden=true"     { hidden = true; }
		if line == "Terminal=true"   { terminal = true; }

		if let Some(v) = line.strip_prefix("OnlyShowIn=")
		{
			only_show_in = Some(parse_semicolon_list(v));
		}
		if let Some(v) = line.strip_prefix("NotShowIn=")
		{
			not_show_in = parse_semicolon_list(v);
		}

		// TryExec: if the key is present, the binary must exist on $PATH.
		if let Some(v) = line.strip_prefix("TryExec=") && try_exec.is_empty()
		{
			try_exec = v.trim().to_string();
		}

		// Locale-aware Name parsing: prefer Name[xx_YY], fall back to Name[xx], then Name.
		if let Some(v) = line.strip_prefix("Name=") && name.is_empty()
		{
			name = v.to_string();
		}
		if !locale.is_empty()
		{
			let key_full  = format!("Name[{}]=", locale);
			let key_short = format!("Name[{}]=", locale_short);
			if let Some(v) = line.strip_prefix(key_full.as_str())  && name_locale.is_empty()       { name_locale = v.to_string(); }
			if let Some(v) = line.strip_prefix(key_short.as_str()) && name_locale_short.is_empty() { name_locale_short = v.to_string(); }
		}

		// GenericName with locale fallback.
		if let Some(v) = line.strip_prefix("GenericName=") && generic_name.is_empty()
		{
			generic_name = v.to_string();
		}
		if !locale.is_empty()
		{
			let key_full  = format!("GenericName[{}]=", locale);
			let key_short = format!("GenericName[{}]=", locale_short);
			if let Some(v) = line.strip_prefix(key_full.as_str())  && generic_locale.is_empty()       { generic_locale = v.to_string(); }
			if let Some(v) = line.strip_prefix(key_short.as_str()) && generic_locale_short.is_empty() { generic_locale_short = v.to_string(); }
		}

		if let Some(v) = line.strip_prefix("Exec=") && exec.is_empty()
		{
			exec = sanitize_exec(v);
		}

		// Comment with locale fallback.
		if let Some(v) = line.strip_prefix("Comment=") && comment.is_empty()
		{
			comment = v.to_string();
		}
		if !locale.is_empty()
		{
			let key_full  = format!("Comment[{}]=", locale);
			let key_short = format!("Comment[{}]=", locale_short);
			if let Some(v) = line.strip_prefix(key_full.as_str())  && comment_locale.is_empty()       { comment_locale = v.to_string(); }
			if let Some(v) = line.strip_prefix(key_short.as_str()) && comment_locale_short.is_empty() { comment_locale_short = v.to_string(); }
		}

		if let Some(v) = line.strip_prefix("Icon=") && icon.is_empty()
		{
			icon = v.to_string();
		}
		if let Some(v) = line.strip_prefix("Keywords=") && keywords.is_empty()
		{
			keywords = parse_keywords(v);
		}
	}

	if no_display || hidden || name.is_empty() || exec.is_empty()
	{
		return None;
	}

	// TryExec: hide the entry if the specified binary cannot be found on $PATH.
	if !try_exec.is_empty() && which::which(&try_exec).is_err()
	{
		return None;
	}

	// OnlyShowIn / NotShowIn filtering against $XDG_CURRENT_DESKTOP.
	if let Some(desktop) = current_desktop
	{
		if let Some(ref only) = only_show_in && !desktop.iter().any(|d| only.iter().any(|o| o.eq_ignore_ascii_case(d)))
		{
		        return None;
		}
		if !not_show_in.is_empty() && desktop.iter().any(|d| not_show_in.iter().any(|n| n.eq_ignore_ascii_case(d)))
		{
			return None;
		}
	}

	// Pick the most specific locale name available.
	let resolved_name = if !name_locale.is_empty()       { name_locale }
	                    else if !name_locale_short.is_empty() { name_locale_short }
	                    else                              { name };

	// Resolve localized GenericName.
	let resolved_generic = if !generic_locale.is_empty()       { generic_locale }
	                       else if !generic_locale_short.is_empty() { generic_locale_short }
	                       else                               { generic_name };

	// Resolve localized Comment.
	let resolved_comment = if !comment_locale.is_empty()       { comment_locale }
	                       else if !comment_locale_short.is_empty() { comment_locale_short }
	                       else                               { comment };

	Some(AppEntry {
		name: resolved_name,
		generic_name: resolved_generic,
		exec,
		comment: resolved_comment,
		icon,
		icon_path: None,
		keywords,
		terminal,
		name_lc: String::new(),
		generic_name_lc: String::new(),
		exec_lc: String::new(),
		comment_lc: String::new(),
		keywords_lc: Vec::new(),
	})
}


/// Parse $XDG_CURRENT_DESKTOP into individual names (colon-separated per spec).
fn current_desktop_names() -> Option<Vec<String>>
{
	let val = std::env::var("XDG_CURRENT_DESKTOP").ok()?;
	if val.is_empty() { return None; }
	Some(val.split(':').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
}


fn parse_semicolon_list(raw: &str) -> Vec<String>
{
	raw.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
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
	let home = home::home_dir().unwrap_or_default();

	// User installs.
	dirs.push(home.join(".local/share/applications"));

	// Flatpak user exports.
	dirs.push(home.join(".local/share/flatpak/exports/share/applications"));

	// XDG_DATA_DIRS (includes system Flatpak exports when set by the session).
	if let Ok(xdg_dirs) = std::env::var("XDG_DATA_DIRS") {
		for path in xdg_dirs.split(':').filter(|s| !s.is_empty()) {
			dirs.push(PathBuf::from(path).join("applications"));
		}
	}

	// Standard system paths.
	dirs.push(PathBuf::from("/usr/local/share/applications"));
	dirs.push(PathBuf::from("/usr/share/applications"));

	// System-wide Flatpak exports (when XDG_DATA_DIRS doesn't include it).
	dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));

	// Distrobox host — apps exported from the host container.
	dirs.push(PathBuf::from("/run/host/usr/share/applications"));
	dirs.push(PathBuf::from("/run/host/usr/local/share/applications"));

	// Deduplicate while preserving priority order.
	let mut seen = std::collections::HashSet::new();
	dirs.retain(|p| seen.insert(p.clone()));

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
	// Preserve the original casing from the .desktop file.
	// The lowercase version is produced later by AppEntry::with_normalized()
	// into keywords_lc, which is what case-insensitive search uses.
	// Lowercasing here would break case-sensitive keyword searches.
	raw.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}
