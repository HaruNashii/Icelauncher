# ☄️ Icelauncher

**Icelauncher** is a lightweight, **Wayland application launcher** written in **Rust**, powered by **iced** and **iced-layer-shell**.

It aims to provide a **minimal, fast, and fully themeable launcher** designed for wlroots and smithay compositors (Hyprland, Sway, Niri, etc.), with real icon support, grid layout, and deep configurability via a RON config file.

> Built for people who want a clean, keyboard-driven launcher without pulling in an entire desktop environment.

---

## ✨ Features

- 🧊 Native Wayland layer-shell launcher
- ⚡ Written entirely in Rust
- 🎨 UI powered by `iced`
- 🖼 Real app icons fetched from the system icon theme
- 🔲 Grid layout support (list or NxN grid)
- ⌨️ Full keyboard navigation (arrows, enter, escape)
- 🔍 Fuzzy-style search across name, comment, exec, and keywords
- 🖥 Terminal app support via configurable terminal command
- 🎛 Fully themeable via RON config (colors, borders, shadows, radii, fonts, spacing...)
- 🪶 Lightweight and compositor-friendly

---

## 🧠 What Icelauncher Does

Icelauncher is **not** a traditional desktop environment launcher.

Instead, it acts as a:

- Wayland **layer-shell surface**
- `.desktop` file scanner and parser
- System icon theme resolver
- Keyboard-driven application picker

### Core responsibilities

- scans `.desktop` files from standard XDG directories
- resolves real app icons from the system icon theme (hicolor, Papirus, Adwaita, Flatpak, etc.)
- renders a searchable, navigable list or grid of applications
- launches the selected app (with optional terminal wrapping)
- closes itself after launch

Conceptually:
```
.desktop Files ──scan──▶ Icelauncher Core
                              │
                              ▼
                       Icon Theme Resolver
                              │
                              ▼
                        iced UI Renderer
                              │
                              ▼
                    Wayland Layer Surface
```

---

## 🖥 Supported Environments

Icelauncher targets **smithay and wlroots based compositors**, including:

- Hyprland
- Sway
- Niri
- Others layer-shell compatible compositors

X11 is **not supported**.

---

## 📦 Tech Stack

- Rust
- iced (GUI framework)
- iced_layershell
- RON (config format)
- tokio (async runtime)

---

## 🚀 Installation

#### **Building From Source:**

Requirements:
- Rust/Cargo (stable/2024 edition)
- gcc-libs

```bash
git clone https://github.com/HaruNashii/icelauncher
cd icelauncher
cargo build --release
mkdir -p $HOME/.local/bin
cp -rf target/release/icelauncher $HOME/.local/bin/
```

**Tip: Bind it to a key in your compositor config for best results.**

- Example (Hyprland):
```
bind = $mod, D, exec, icelauncher
```

- Example (Sway):
```
bindsym $mod+d exec icelauncher
```

---

## 🎨 Configuration

On first launch, Icelauncher generates a default config at:
```
~/.config/icelauncher/config.ron
```

The config is fully documented inline. Every visual and behavioural property is tuneable.

### Config Sections

| Section | What it controls |
|---|---|
| `window` | Size, padding, grid columns, border, shadow, background |
| `search` | Placeholder, font size, colors, border |
| `entry` | Name/comment font, padding, hover/selected/pressed colors |
| `icon` | Real vs abstract icons, badge size, colors, border |
| `footer` | Hint text, result count, colors |
| `behaviour` | Search fields, case sensitivity, terminal command, close on launch |

### Color Formats

All color values support three formats:
```ron
RGB((255, 255, 255))
RGBA((255, 255, 255, 80))   // alpha is 0–100
HEX("3d3d3d")               // 6-digit or 8-digit hex
```

### Grid Layout

Set `grid_side_items` in the `window` section to control columns:
```ron
grid_side_items: 1   // classic list
grid_side_items: 3   // 3-column grid
```

### Real Icons

Set `use_real_icons: true` in the `icon` section to fetch icons from your system theme.
Icelauncher searches hicolor, your active GTK theme, Adwaita, Papirus, Flatpak exports, and more automatically.

### Terminal Apps

To support `.desktop` entries with `Terminal=true`, set your terminal command:
```ron
terminal_command: "kitty -e"
// or: "alacritty -e" | "foot" | "wezterm -e"
```

---

## 🧩 Architecture Overview

```
src/
├── main.rs          → application entry point + AppData + Message
├── subscription.rs  → iced subscriptions (keyboard events)
├── update.rs        → iced message handler
├── view.rs          → iced renderer
├── ron.rs           → RON config structs + loader
├── helpers/*.rs     → app scanning, icon resolution, search, style helpers
```

### Key Systems

**1. Layer Shell Integration**
- Creates a centered Wayland overlay surface without a desktop environment.

**2. .desktop Scanner**
- Reads XDG application directories including user installs, system installs, Flatpak, and Distrobox host paths.

**3. Icon Resolver**
- Walks the system icon theme tree (standard layout, alternate layout, Flatpak exports, pixmaps) to find the best matching icon for each app.

**4. Search Engine**
- Scores and ranks results across name, comment, exec, and keywords with configurable field toggles and case sensitivity.

**5. Event Model**
- Follows iced's update/view architecture:
  - Message → Update → State → View

---

## ⚠️ Current Status

Experimental / Work in Progress.
Expect:
- breaking changes
- incomplete features
- rapid iteration

---

## 🪲 Known Bugs

- Crashing on GNOME
  - Explanation: Icelauncher depends on [Layer Shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1#compositor-support) which GNOME has not implemented yet.

---

## 🛠 Roadmap (Planned Ideas)

- Plugin/custom module API
- Better icon caching
- Animations (very low priority)

---

## 🤝 Contributing

**Contributions are welcome!**

Good areas to help:
- Wayland handling
- iced widgets
- icon theme compatibility
- performance improvements
- compositor testing

**Steps:**
```
fork → branch → commit → pull request
```

---

## 📜 License

MIT License.
See [LICENSE](LICENSE) for details.
