// ============ IMPORTS ============
use std::path::Path;




// ============ STATIC'S/CONST'S ============
const SIZES: &[&str] = &[
	"scalable", "512x512", "256x256", "128x128", "96x96", "72x72",
	"64x64", "48x48", "36x36", "32x32", "24x24", "22x22", "16x16",
];
const SIZES_FLAT: &[&str] = &[
	"512", "256", "128", "96", "72", "64", "48", "36", "32", "24", "22", "16",
];
const ICON_MAP: &[(&[&str], &str)] = &[
	(&["terminal", "alacritty", "kitty", "foot"], "⊞"),
	(&["firefox", "chromium", "browser", "chrome"], "🌐"),
	(&["music", "spotify", "rhythmbox", "mpv"], "♫"),
	(&["file", "nautilus", "dolphin", "thunar"], "📁"),
	(&["text", "gedit", "notepad", "kate"], "📝"),
	(&["image", "photo", "gimp", "inkscape"], "🖼"),
	(&["code", "vscode", "vscodium"], "</>"),
	(&["setting", "control", "system"], "⚙"),
	(&["mail", "thunderbird", "email"], "✉"),
	(&["calc", "math"], "🧮"),
];




// ============ FUNCTIONS ============
pub fn resolve_icon_with(icon: &str, bases: &[String], themes: &[String]) -> Option<String>
{
	if icon.is_empty() {
		return None;
	}
	if icon.starts_with('/') {
		return Path::new(icon).exists().then(|| icon.to_string());
	}

	let name = icon
		.strip_suffix(".png")
		.or_else(|| icon.strip_suffix(".svg"))
		.or_else(|| icon.strip_suffix(".xpm"))
		.unwrap_or(icon);

	let mut buf = String::with_capacity(256);

	for base in bases {
		for theme in themes {
			for sz in SIZES {
				for ext in ["svg", "png"] {
					build_path4(&mut buf, base, theme, sz, "apps", name, ext);
					if Path::new(&buf).exists() {
						return Some(buf);
					}
				}
			}
			for ext in ["svg", "png"] {
				build_path3(&mut buf, base, theme, "apps/scalable", name, ext);
				if Path::new(&buf).exists() {
					return Some(buf);
				}
			}
			for sz in SIZES_FLAT {
				for ext in ["svg", "png"] {
					build_path4(&mut buf, base, theme, "apps", sz, name, ext);
					if Path::new(&buf).exists() {
						return Some(buf);
					}
				}
			}
		}
	}

	let home = home::home_dir().unwrap_or_default();
	let home_str = home.to_string_lossy();
	let host_user = std::env::var("USER").unwrap_or_default();

	let flatpak_candidates: [(&str, &str); 8] = [
		(&home_str, ".local/share/flatpak/exports/share/icons/hicolor/scalable/apps"),
		(&home_str, ".local/share/flatpak/exports/share/icons/hicolor/48x48/apps"),
		("/var/lib/flatpak/exports/share/icons/hicolor/scalable/apps", ""),
		("/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps", ""),
		("/run/host/usr/share/icons/hicolor/scalable/apps", ""),
		("/run/host/usr/share/icons/hicolor/48x48/apps", ""),
		("/run/host/var/lib/flatpak/exports/share/icons/hicolor/scalable/apps", ""),
		("/run/host/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps", ""),
	];

	let host_paths = [
		format!("/run/host/home/{host_user}/.local/share/flatpak/exports/share/icons/hicolor/scalable/apps"),
		format!("/run/host/home/{host_user}/.local/share/flatpak/exports/share/icons/hicolor/48x48/apps"),
	];

	for (prefix, suffix) in &flatpak_candidates {
		for ext in ["svg", "png"] {
			buf.clear();
			buf.push_str(prefix);
			if !suffix.is_empty() {
				buf.push('/');
				buf.push_str(suffix);
			}
			buf.push('/');
			buf.push_str(name);
			buf.push('.');
			buf.push_str(ext);
			if Path::new(&buf).exists() {
				return Some(buf);
			}
		}
	}

	for dir in &host_paths {
		for ext in ["svg", "png"] {
			buf.clear();
			buf.push_str(dir);
			buf.push('/');
			buf.push_str(name);
			buf.push('.');
			buf.push_str(ext);
			if Path::new(&buf).exists() {
				return Some(buf);
			}
		}
	}

	for ext in ["png", "svg"] {
		buf.clear();
		buf.push_str("/usr/share/pixmaps/");
		buf.push_str(name);
		buf.push('.');
		buf.push_str(ext);
		if Path::new(&buf).exists() {
			return Some(buf);
		}
	}

	buf.clear();
	buf.push_str("/usr/share/icons/hicolor/scalable/apps/");
	buf.push_str(name);
	buf.push_str(".svg");
	if Path::new(&buf).exists() {
		return Some(buf);
	}

	None
}


pub fn icon_base_dirs() -> Vec<String>
{
	let mut dirs = Vec::new();

	if let Some(home) = home::home_dir() {
		dirs.push(home.join(".local/share/icons").to_string_lossy().into_owned());
		dirs.push(home.join(".local/share/flatpak/exports/share/icons").to_string_lossy().into_owned());
	}

	if let Ok(xdg) = std::env::var("XDG_DATA_DIRS") {
		for p in xdg.split(':') {
			dirs.push(format!("{}/icons", p.trim_end_matches('/')));
		}
	}

	dirs.push("/var/lib/flatpak/exports/share/icons".to_string());
	dirs.push("/usr/share/icons".to_string());
	dirs.push("/usr/local/share/icons".to_string());
	dirs.push("/run/host/usr/share/icons".to_string());
	dirs.push("/run/host/usr/local/share/icons".to_string());

	let mut seen = std::collections::HashSet::new();
	dirs.retain(|d| seen.insert(d.clone()));
	dirs
}


pub fn discover_themes(bases: &[String]) -> Vec<String>
{
	let preferred = get_icon_theme();
	let mut themes: Vec<String> = Vec::new();

	if !preferred.is_empty() {
		themes.push(preferred.clone());
	}
	for base in bases {
		if let Ok(entries) = std::fs::read_dir(base) {
			for entry in entries.flatten() {
				let name = entry.file_name().to_string_lossy().into_owned();
				let index = entry.path().join("index.theme");
				if index.exists()
					&& name != preferred
					&& name != "hicolor"
					&& !themes.contains(&name)
				{
					themes.push(name);
				}
			}
		}
	}

	if !themes.contains(&"hicolor".to_string()) {
		themes.push("hicolor".to_string());
	}

	eprintln!("[icelauncher] Themes found: {:?}", themes);
	themes
}


pub fn derive_icon_char(name: &str) -> &'static str
{
	let lower = name.to_lowercase();
	for (keywords, glyph) in ICON_MAP {
		if keywords.iter().any(|k| lower.contains(k)) {
			return glyph;
		}
	}
	"▶"
}


fn get_icon_theme() -> String
{
	if let Some(home) = home::home_dir() {
		for cfg in &["gtk-4.0/settings.ini", "gtk-3.0/settings.ini"] {
			let path = home.join(".config").join(cfg);
			if let Ok(text) = std::fs::read_to_string(path) {
				for line in text.lines() {
					if let Some(v) = line.strip_prefix("gtk-icon-theme-name=") {
						let theme = v.trim().to_string();
						eprintln!("[icelauncher] Theme selected: {} (from GTK)", theme);
						return theme;
					}
				}
			}
		}

		let kdeglobals = home.join(".config/kdeglobals");
		if let Ok(text) = std::fs::read_to_string(kdeglobals) {
			let mut in_icons = false;
			for line in text.lines() {
				if line.trim() == "[Icons]" {
					in_icons = true;
					continue;
				}
				if line.starts_with('[') {
					in_icons = false;
					continue;
				}
				if in_icons && let Some(v) = line.strip_prefix("Theme=") {
					let theme = v.trim().to_string();
					eprintln!("[icelauncher] Theme selected: {} (from KDE)", theme);
					return theme;
				}
			}
		}

		let xsettings = home.join(".config/xfce4/xfconf/xfce-perchannel-xml/xsettings.xml");
		if let Ok(text) = std::fs::read_to_string(xsettings) {
			for line in text.lines() {
				let line = line.trim();
				if line.contains("\"Net/IconThemeName\"")
					&& let Some(start) = line.find("value=\"")
				{
					let rest = &line[start + 7..];
					if let Some(end) = rest.find('"') {
						let theme = rest[..end].to_string();
						eprintln!("[icelauncher] Theme selected: {} (from XFCE)", theme);
						return theme;
					}
				}
			}
		}
	}

	eprintln!("[icelauncher] Theme selected: hicolor (fallback)");
	"hicolor".to_string()
}


#[inline(always)]
fn build_path4(buf: &mut String, a: &str, b: &str, c: &str, d: &str, name: &str, ext: &str)
{
	buf.clear();
	buf.push_str(a); buf.push('/');
	buf.push_str(b); buf.push('/');
	buf.push_str(c); buf.push('/');
	buf.push_str(d); buf.push('/');
	buf.push_str(name); buf.push('.'); buf.push_str(ext);
}


#[inline(always)]
fn build_path3(buf: &mut String, a: &str, b: &str, c: &str, name: &str, ext: &str)
{
	buf.clear();
	buf.push_str(a); buf.push('/');
	buf.push_str(b); buf.push('/');
	buf.push_str(c); buf.push('/');
	buf.push_str(name); buf.push('.'); buf.push_str(ext);
}
