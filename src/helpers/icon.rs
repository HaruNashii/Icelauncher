// ============ IMPORTS ============
use std::path::Path;
use crate::ron::IconConfig;




// ============ STATIC'S/CONST'S ============
const SIZES: &[&str] = &
[
	"scalable", "512x512", "256x256", "128x128", "96x96", "72x72",
	"64x64", "48x48", "36x36", "32x32", "24x24", "22x22", "16x16",
];
const SIZES_FLAT: &[&str] = &
[
	"512", "256", "128", "96", "72", "64", "48", "36", "32", "24", "22", "16",
];




// ============ FUNCTIONS ============
fn define_icon_map(icons: IconConfig) -> [(Vec<&'static str>, String); 12]
{
    [
    	(vec!["terminal", "konsole", "tilix", "xterm", "wezterm", "xfce4-terminal", "terminator", "alacritty", "kitty", "foot", "kgx", "gnome-terminal", "ghostty"], icons.terminal_generic_icon),
    	(vec!["firefox", "chromium", "browser", "chrome", "zen", "zen-browser", "brave", "vivaldi", "opera", "tor-browser", "edge", "librewolf", "epiphany"], icons.browser_generic_icon),
    	(vec!["music", "audacious", "strawberry", "lollypop", "amberol", "spotify", "rhythmbox", "mpv"], icons.music_player_generic_icon),
    	(vec!["file", "nemo", "pacmanfm", "caja", "krusader", "ranger", "nautilus", "dolphin", "thunar"], icons.file_manager_generic_icon),
    	(vec!["text", "gedit", "notepad", "kate"], icons.text_editor_generic_icon),
    	(vec!["image", "gwenview", "gthumb",  "feh", "eog", "vlc", "photo", "gimp", "inkscape", "krita"], icons.media_viewer_generic_icon),
    	(vec!["code", "vscode", "vscodium", "vim", "neovim", "cmake"], icons.code_generic_icon),
    	(vec!["mail", "thunderbird", "email", "geary", "evolution", "mailspring"], icons.mail_generic_icon),
    	(vec!["calc", "math", "kcalc", "calculator"], icons.calc_generic_icon),
        (vec!["setting", "control", "adwsteamgtk", "system", "tweaks", "lxappearance", "nwg-look", "GTK Settings", "customize look and feel"], icons.setting_generic_icon),
    	(vec!["game", "steam", "lutris", "heroic", "epic", "epic_games", "EA app", "Battle", "GOG", "hydra", "fugus"], icons.game_generic_icon),
        (vec!["discord", "telegram"], icons.social_media_generic_icon)
    ]
}



pub fn resolve_icon_with(icon: &str, bases: &[String], themes: &[String]) -> Option<String>
{
	if icon.is_empty() 
        {
		return None;
	}
	if icon.starts_with('/') 
        {
		return Path::new(icon).exists().then(|| icon.to_string());
	}

	let name = icon
		.strip_suffix(".png")
		.or_else(|| icon.strip_suffix(".svg"))
		.or_else(|| icon.strip_suffix(".xpm"))
		.unwrap_or(icon);

	let mut buf = String::with_capacity(256);

	for base in bases 
        {
		for theme in themes 
                {
			for sz in SIZES 
                        {
				for ext in ["svg", "png"] 
                                {
					build_path4(&mut buf, base, theme, sz, "apps", name, ext);
					if Path::new(&buf).exists() 
                                        {
						return Some(buf);
					}
				}
			}
			for ext in ["svg", "png"] 
                        {
				build_path3(&mut buf, base, theme, "apps/scalable", name, ext);
				if Path::new(&buf).exists() 
                                {
					return Some(buf);
				}
			}
			for sz in SIZES_FLAT 
                        {
				for ext in ["svg", "png"] 
                                {
					build_path4(&mut buf, base, theme, "apps", sz, name, ext);
					if Path::new(&buf).exists() 
                                        {
						return Some(buf);
					}
				}
			}
		}
	}

	let home = home::home_dir().unwrap_or_default();
	let home_str = home.to_string_lossy();
	let host_user = std::env::var("USER").unwrap_or_default();

	let flatpak_candidates: [(&str, &str); 8] = 
        [
		(&home_str, ".local/share/flatpak/exports/share/icons/hicolor/scalable/apps"),
		(&home_str, ".local/share/flatpak/exports/share/icons/hicolor/48x48/apps"),
		("/var/lib/flatpak/exports/share/icons/hicolor/scalable/apps", ""),
		("/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps", ""),
		("/run/host/usr/share/icons/hicolor/scalable/apps", ""),
		("/run/host/usr/share/icons/hicolor/48x48/apps", ""),
		("/run/host/var/lib/flatpak/exports/share/icons/hicolor/scalable/apps", ""),
		("/run/host/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps", ""),
	];

	let host_paths = 
        [
		format!("/run/host/home/{host_user}/.local/share/flatpak/exports/share/icons/hicolor/scalable/apps"),
		format!("/run/host/home/{host_user}/.local/share/flatpak/exports/share/icons/hicolor/48x48/apps"),
	];

	for (prefix, suffix) in &flatpak_candidates 
        {
		for ext in ["svg", "png"] 
                {
			buf.clear();
			buf.push_str(prefix);
			if !suffix.is_empty() 
                        {
				buf.push('/');
				buf.push_str(suffix);
			}
			buf.push('/');
			buf.push_str(name);
			buf.push('.');
			buf.push_str(ext);
			if Path::new(&buf).exists() 
                        {
				return Some(buf);
			}
		}
	}

	for dir in &host_paths 
        {
		for ext in ["svg", "png"] 
                {
			buf.clear();
			buf.push_str(dir);
			buf.push('/');
			buf.push_str(name);
			buf.push('.');
			buf.push_str(ext);
			if Path::new(&buf).exists() 
                        {
				return Some(buf);
			}
		}
	}

	for ext in ["png", "svg"] 
        {
		buf.clear();
		buf.push_str("/usr/share/pixmaps/");
		buf.push_str(name);
		buf.push('.');
		buf.push_str(ext);
		if Path::new(&buf).exists() 
                {
			return Some(buf);
		}
	}

	buf.clear();
	buf.push_str("/usr/share/icons/hicolor/scalable/apps/");
	buf.push_str(name);
	buf.push_str(".svg");
	if Path::new(&buf).exists() 
        {
		return Some(buf);
	}

	None
}



pub fn icon_base_dirs() -> Vec<String>
{
	let mut dirs = Vec::new();

	if let Some(home) = home::home_dir() 
        {
		dirs.push(home.join(".local/share/icons").to_string_lossy().into_owned());
		dirs.push(home.join(".local/share/flatpak/exports/share/icons").to_string_lossy().into_owned());
	}

	if let Ok(xdg) = std::env::var("XDG_DATA_DIRS") 
        {
		for p in xdg.split(':') 
                {
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

	if !preferred.is_empty() 
        {
		expand_theme_chain(&preferred, bases, &mut themes);
	}

	for base in bases 
        {
		if let Ok(entries) = std::fs::read_dir(base) 
                {
			for entry in entries.flatten() 
                        {
				let name = entry.file_name().to_string_lossy().into_owned();
				let index = entry.path().join("index.theme");
				if index.exists() && name != "hicolor" && !themes.contains(&name)
				{
					themes.push(name);
				}
			}
		}
	}

	if !themes.contains(&"hicolor".to_string()) 
        {
		themes.push("hicolor".to_string());
	}

	themes
}



fn expand_theme_chain(theme: &str, bases: &[String], out: &mut Vec<String>)
{
	if out.contains(&theme.to_string()) 
        {
		return;
	}
	out.push(theme.to_string());

	let parents = read_theme_parents(theme, bases);
	for parent in parents 
        {
		if parent != "hicolor" 
                {
			expand_theme_chain(&parent, bases, out);
		}
	}
}



fn read_theme_parents(theme: &str, bases: &[String]) -> Vec<String>
{
	for base in bases 
        {
		let index_path = format!("{}/{}/index.theme", base, theme);
		let Ok(text) = std::fs::read_to_string(&index_path) else { continue };
		for line in text.lines() 
                {
			if let Some(val) = line.strip_prefix("Inherits=") 
                        {
				return val
					.split(',')
					.map(|s| s.trim().to_string())
					.filter(|s| !s.is_empty())
					.collect();
			}
		}
	}
	vec![]
}



pub fn derive_icon_char(name: &str, config: IconConfig) -> String
{
	let lower = name.to_lowercase();
	for (keywords, glyph) in define_icon_map(config.clone())
        {
		if keywords.iter().any(|k| lower.contains(k)) 
                {
			return glyph;
		}
	}
        config.generic_icon
}


fn get_icon_theme() -> String
{
	if let Some(home) = home::home_dir() 
        {
		for cfg in &["gtk-4.0/settings.ini", "gtk-3.0/settings.ini"] 
                {
			let path = home.join(".config").join(cfg);
			if let Ok(text) = std::fs::read_to_string(path) 
                        {
				for line in text.lines() 
                                {
					if let Some(v) = line.strip_prefix("gtk-icon-theme-name=") 
                                        {
						let theme = v.trim().to_string();
						return theme;
					}
				}
			}
		}

		let kdeglobals = home.join(".config/kdeglobals");
		if let Ok(text) = std::fs::read_to_string(kdeglobals) 
                {
			let mut in_icons = false;
			for line in text.lines() 
                        {
				if line.trim() == "[Icons]" 
                                {
					in_icons = true;
					continue;
				}
				if line.starts_with('[') 
                                {
					in_icons = false;
					continue;
				}
				if in_icons && let Some(v) = line.strip_prefix("Theme=") 
                                {
					let theme = v.trim().to_string();
					return theme;
				}
			}
		}

		let xsettings = home.join(".config/xfce4/xfconf/xfce-perchannel-xml/xsettings.xml");
		if let Ok(text) = std::fs::read_to_string(xsettings) 
                {
			for line in text.lines() 
                        {
				let line = line.trim();
				if line.contains("\"Net/IconThemeName\"") && let Some(start) = line.find("value=\"")
				{
					let rest = &line[start + 7..];
					if let Some(end) = rest.find('"') 
                                        {
						let theme = rest[..end].to_string();
						return theme;
					}
				}
			}
		}
	}

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
