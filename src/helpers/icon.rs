// ============ IMPORTS ============
use std::path::Path;




// ============ FUNCTIONS ============
pub fn resolve_icon_with(icon: &str, bases: &[String], themes: &[String]) -> Option<String>
{
    if icon.is_empty() { return None; }
    if icon.starts_with('/')
    {
        if Path::new(icon).exists() { return Some(icon.to_string()); }
        return None;
    }

    let name = icon.strip_suffix(".png")
        .or_else(|| icon.strip_suffix(".svg"))
        .or_else(|| icon.strip_suffix(".xpm"))
        .unwrap_or(icon);

    for base in bases
    {
        for theme in themes
        {
            let theme_root = format!("{}/{}", base, theme);

            let standard_sizes =
            [
                "scalable", "512x512", "256x256", "128x128", "96x96",
                "72x72", "64x64", "48x48", "36x36", "32x32", "24x24", "22x22", "16x16"
            ];

            for sz in standard_sizes
            {
                for ext in ["svg", "png"]
                {
                    let p = format!("{}/{}/apps/{}.{}", theme_root, sz, name, ext);
                    if Path::new(&p).exists() { return Some(p); }
                }
            }

            for ext in ["svg", "png"]
            {
                let p = format!("{}/apps/scalable/{}.{}", theme_root, name, ext);
                if Path::new(&p).exists() { return Some(p); }
            }

            for sz in ["512", "256", "128", "96", "72", "64", "48", "36", "32", "24", "22", "16"]
            {
                for ext in ["svg", "png"]
                {
                    let p = format!("{}/apps/{}/{}.{}", theme_root, sz, name, ext);
                    if Path::new(&p).exists() { return Some(p); }
                }
            }
        }
    }

    let home      = home::home_dir().unwrap_or_default();
    let home_str  = home.to_string_lossy();
    let host_user = std::env::var("USER").unwrap_or_default();

    let flatpak_candidates =
    [
        format!("{home_str}/.local/share/flatpak/exports/share/icons/hicolor/scalable/apps/{name}.svg"),
        format!("{home_str}/.local/share/flatpak/exports/share/icons/hicolor/48x48/apps/{name}.png"),
        format!("/var/lib/flatpak/exports/share/icons/hicolor/scalable/apps/{name}.svg"),
        format!("/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps/{name}.png"),
        format!("/run/host/home/{host_user}/.local/share/flatpak/exports/share/icons/hicolor/scalable/apps/{name}.svg"),
        format!("/run/host/home/{host_user}/.local/share/flatpak/exports/share/icons/hicolor/48x48/apps/{name}.png"),
        format!("/run/host/var/lib/flatpak/exports/share/icons/hicolor/scalable/apps/{name}.svg"),
        format!("/run/host/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps/{name}.png"),
    ];

    for p in &flatpak_candidates
    {
        if Path::new(p).exists() { return Some(p.clone()); }
    }

    for ext in ["png", "svg"]
    {
        let p = format!("/usr/share/pixmaps/{}.{}", name, ext);
        if Path::new(&p).exists() { return Some(p); }
    }

    let p = format!("/usr/share/icons/hicolor/scalable/apps/{}.svg", name);
    if Path::new(&p).exists() { return Some(p); }

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
    let preferred  = get_icon_theme();
    let mut themes: Vec<String> = Vec::new();

    if !preferred.is_empty() { themes.push(preferred.clone()); }
    for base in bases
    {
        if let Ok(entries) = std::fs::read_dir(base)
        {
            for entry in entries.flatten()
            {
                let name  = entry.file_name().to_string_lossy().into_owned();
                let index = entry.path().join("index.theme");
                if index.exists() && name != preferred && name != "hicolor" && !themes.contains(&name) 
                { 
                    themes.push(name); 
                }
            }
        }
    }

    if !themes.contains(&"hicolor".to_string()) { themes.push("hicolor".to_string()); }

    println!("[icelauncher] Themes found: {:?}", themes);

    themes
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
                        println!("[icelauncher] Theme selected: {} (from GTK)", theme);
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
                if line.trim() == "[Icons]" { in_icons = true;  continue; }
                if line.starts_with('[')    { in_icons = false; continue; }
                if in_icons && let Some(v) = line.strip_prefix("Theme=")
                {
                    let theme = v.trim().to_string();
                    println!("[icelauncher] Theme selected: {} (from KDE)", theme);
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
                            println!("[icelauncher] Theme selected: {} (from XFCE)", theme);
                            return theme;
                        }
                }
            }
        }
    }

    println!("[icelauncher] Theme selected: hicolor (fallback)");
    "hicolor".to_string()
}



pub fn derive_icon_char(name: &str) -> &'static str
{
    let lower = name.to_lowercase();
    if lower.contains("terminal")    || lower.contains("alacritty")   || lower.contains("kitty")     || lower.contains("foot")     { return "⊞"; }
    if lower.contains("firefox")     || lower.contains("chromium")    || lower.contains("browser")   || lower.contains("chrome")   { return "🌐"; }
    if lower.contains("music")       || lower.contains("spotify")     || lower.contains("rhythmbox")  || lower.contains("mpv")     { return "♫"; }
    if lower.contains("file")        || lower.contains("nautilus")    || lower.contains("dolphin")   || lower.contains("thunar")   { return "📁"; }
    if lower.contains("text")        || lower.contains("gedit")       || lower.contains("notepad")   || lower.contains("kate")     { return "📝"; }
    if lower.contains("image")       || lower.contains("photo")       || lower.contains("gimp")      || lower.contains("inkscape") { return "🖼"; }
    if lower.contains("code")        || lower.contains("vscode")      || lower.contains("vscodium")                                { return "</>"; }
    if lower.contains("setting")     || lower.contains("control")     || lower.contains("system")                                  { return "⚙"; }
    if lower.contains("mail")        || lower.contains("thunderbird") || lower.contains("email")                                   { return "✉"; }
    if lower.contains("calc")        || lower.contains("math")                                                                     { return "🧮"; }
    "▶"
}
