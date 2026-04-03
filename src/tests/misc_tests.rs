use tempfile::{tempdir, NamedTempFile};

use crate::helpers::icon::resolve_icon;
use crate::ron::{LauncherConfig, config_path, load_config};
use crate::{update, Message, AppData, AppEntry};


pub fn entry(name: &str) -> AppEntry
{
    AppEntry
    {
        name:      name.to_string(),
        exec:      name.to_string(),
        comment:   String::new(),
        icon:      String::new(),
        icon_path: None,
        keywords:  vec![],
        terminal:  false,
    }
}

pub fn make_app_with_entries(entries: Vec<AppEntry>, cols: usize, max: usize) -> AppData
{
    let mut cfg = LauncherConfig::default();
    cfg.window.grid_side_items = cols;
    cfg.window.max_results     = max;

    AppData
    {
        filtered: entries.clone(),
        entries,
        config: cfg, // IMPORTANT: you forgot this earlier in your tests
        loading: false,
        ..Default::default()
    }
}



// ── resolve_icon ──────────────────────────────────────────────────────────────

#[test]
fn resolve_icon_empty_string_returns_none()
{
    assert!(resolve_icon("").is_none());
}

#[test]
fn resolve_icon_absolute_path_existing_returns_some()
{
    // Create a real temp file and pass its absolute path
    let f = NamedTempFile::new().unwrap();
    let path = f.path().to_str().unwrap().to_string();
    let result = resolve_icon(&path);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), path);
}

#[test]
fn resolve_icon_absolute_path_missing_returns_none()
{
    let result = resolve_icon("/nonexistent/path/to/icon.png");
    assert!(result.is_none());
}

#[test]
fn resolve_icon_strips_png_suffix_before_searching()
{
    // Passing "icon.png" should internally strip ".png" → searches for "icon"
    // We can't assert it finds something in CI, but it must not panic
    let _ = resolve_icon("icon.png");
}

#[test]
fn resolve_icon_strips_svg_suffix_before_searching()
{
    let _ = resolve_icon("icon.svg");
}

#[test]
fn resolve_icon_strips_xpm_suffix_before_searching()
{
    let _ = resolve_icon("icon.xpm");
}

#[test]
fn resolve_icon_unknown_name_returns_none()
{
    // A name that surely doesn't exist on any system
    let result = resolve_icon("__icelauncher_test_nonexistent_icon_xyzzy__");
    assert!(result.is_none());
}




// ── config_path ──────────────────────────────────────────────────────────────

#[test]
fn config_path_ends_with_expected_suffix()
{
    let p = config_path();
    let s = p.to_string_lossy();
    assert!(
        s.ends_with(".config/icelauncher/config.ron"),
        "unexpected config path: {s}"
    );
}

#[test]
fn config_path_is_absolute()
{
    assert!(config_path().is_absolute());
}




// ── load_config ───────────────────────────────────────────────────────────────

#[test]
fn load_config_returns_default_when_file_missing()
{
    // Point HOME to a fresh temp dir so no config exists
    let dir = tempdir().unwrap();
    unsafe{std::env::set_var("HOME", dir.path());};

    let cfg = load_config();

    // Should be sane defaults, not a panic
    assert!(cfg.window.width  > 0);
    assert!(cfg.window.height > 0);
}

#[test]
fn load_config_falls_back_to_default_on_malformed_ron()
{
    let dir = tempdir().unwrap();
    unsafe{std::env::set_var("HOME", dir.path());};

    // Write garbage into config path
    let cfg_dir = dir.path().join(".config/icelauncher");
    std::fs::create_dir_all(&cfg_dir).unwrap();
    let cfg_file = cfg_dir.join("config.ron");
    std::fs::write(&cfg_file, b"this is not valid ron }{{{").unwrap();

    let cfg = load_config();
    assert!(cfg.window.width > 0); // falls back to defaults
}

#[test]
fn load_config_parses_valid_minimal_ron()
{
    let dir = tempdir().unwrap();
    unsafe{std::env::set_var("HOME", dir.path());};

    let cfg_dir = dir.path().join(".config/icelauncher");
    std::fs::create_dir_all(&cfg_dir).unwrap();

    // Write a minimal valid config using ron serialization of defaults
    let default_cfg = LauncherConfig::default();
    let serialized  = ron::to_string(&default_cfg).expect("default config must serialize");
    std::fs::write(cfg_dir.join("config.ron"), serialized.as_bytes()).unwrap();

    let cfg = load_config();
    assert_eq!(cfg.window.width,  default_cfg.window.width);
    assert_eq!(cfg.window.height, default_cfg.window.height);
}




// ── LauncherConfig defaults ───────────────────────────────────────────────────

#[test]
fn default_config_window_dimensions_nonzero()
{
    let cfg = LauncherConfig::default();
    assert!(cfg.window.width  > 0);
    assert!(cfg.window.height > 0);
}

#[test]
fn default_config_max_results_nonzero()
{
    let cfg = LauncherConfig::default();
    assert!(cfg.window.max_results > 0);
}

#[test]
fn default_config_grid_side_items_at_least_one()
{
    let cfg = LauncherConfig::default();
    assert!(cfg.window.grid_side_items >= 1);
}

#[test]
fn default_config_entry_sizes_nonzero()
{
    let cfg = LauncherConfig::default();
    assert!(cfg.entry.name_size    > 0);
    assert!(cfg.entry.comment_size > 0);
}

#[test]
fn default_config_search_name_enabled()
{
    // Searching by name must be on by default
    let cfg = LauncherConfig::default();
    assert!(cfg.behaviour.search_name);
}

#[test]
fn default_config_placeholder_nonempty()
{
    let cfg = LauncherConfig::default();
    assert!(!cfg.search.placeholder.is_empty());
}




// ── scroll edge case: partial last row in grid ────────────────────────────────

fn plain_entry(name: &str) -> AppEntry
{
    AppEntry
    {
        name:      name.to_string(),
        exec:      name.to_string(),
        comment:   String::new(),
        icon:      String::new(),
        icon_path: None,
        keywords:  vec![],
        terminal:  false,
    }
}

fn make_scroll_app(entries: Vec<AppEntry>, selected: usize, cols: usize, max: usize, viewport_h: f32, content_h: f32, offset: f32) -> AppData
{
    let mut cfg = LauncherConfig::default();
    cfg.window.grid_side_items = cols;
    cfg.window.max_results     = max;
    cfg.window.entry_spacing   = 3;
    cfg.entry.name_size        = 14;
    cfg.entry.comment_size     = 11;
    cfg.entry.padding          = [6, 10];
    cfg.entry.show_comment     = true;

    AppData
    {
        filtered:      entries.clone(),
        entries,
        selected,
        config:        cfg,
        viewport_h,
        content_h,
        scroll_offset: offset,
        ..Default::default()
    }
}

#[test]
fn scroll_partial_last_row_grid_does_not_panic()
{
    use crate::helpers::scroll::scroll_to_selected;

    // 5 entries, 2 cols → rows: [0,1], [2,3], [4] — partial last row
    let entries: Vec<_> = (0..5).map(|i| plain_entry(&format!("App{i}"))).collect();
    let base_h  = 14_f32 + 6.0 * 2.0 + 8.0; // 34
    let spacing = 3.0;
    let content = base_h * 3.0 + spacing * 2.0;
    let viewport = base_h; // 1 row visible

    // Select the lone item in the last row (index 4)
    let mut app = make_scroll_app(entries, 4, 2, 5, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app); // must not panic
    assert!(app.scroll_offset <= 1.0);
    assert!(app.scroll_offset >= 0.0);
}

#[test]
fn scroll_partial_last_row_scrolls_into_view()
{
    use crate::helpers::scroll::scroll_to_selected;

    let entries: Vec<_> = (0..5).map(|i| plain_entry(&format!("App{i}"))).collect();
    let base_h  = 14_f32 + 6.0 * 2.0 + 8.0;
    let spacing = 3.0;
    let content  = base_h * 3.0 + spacing * 2.0;
    let viewport = base_h;

    let mut app = make_scroll_app(entries, 4, 2, 5, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);

    // Last row (row 2) was below viewport — should have scrolled down
    assert!(app.scroll_offset > 0.0, "expected scroll down for partial last row");
}

#[test]
fn scroll_first_entry_from_bottom_reaches_near_zero()
{
    use crate::helpers::scroll::scroll_to_selected;

    let entries: Vec<_> = (0..8).map(|i| plain_entry(&format!("App{i}"))).collect();
    let base_h  = 14_f32 + 6.0 * 2.0 + 8.0;
    let spacing = 3.0;
    let content  = base_h * 8.0 + spacing * 7.0;
    let viewport = base_h * 2.0; // 2 rows visible

    // Start scrolled to bottom, select first entry
    let mut app = make_scroll_app(entries, 0, 1, 8, viewport, content, 1.0);
    let _ = scroll_to_selected(&mut app);

    assert!(app.scroll_offset < 0.1, "first item should scroll near top, got {}", app.scroll_offset);
}

#[test]
fn selected_never_out_of_bounds_after_navigation()
{
    let entries: Vec<_> = (0..10).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 3, 7);

    for _ in 0..50
    {
        let _ = update(&mut app, Message::SelectDown);
        let _ = update(&mut app, Message::SelectUp);
        let _ = update(&mut app, Message::SelectLeft);
        let _ = update(&mut app, Message::SelectRight);

        let max = app.filtered.len().min(app.config.window.max_results);
        if max > 0
        {
            assert!(app.selected < max);
        }
    }
}

#[test]
fn query_change_clamps_selection()
{
    let mut app = make_app_with_entries(
        vec![entry("Firefox"), entry("Vim"), entry("Nano")],
        1, 10
    );

    app.selected = 2;

    let _ = update(&mut app, Message::QueryChanged("vim".into()));

    assert_eq!(app.selected, 0);
    assert_eq!(app.filtered.len(), 1);
}

#[test]
fn repeated_down_wrap_is_stable()
{
    let entries: Vec<_> = (0..4).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 4);

    for _ in 0..20
    {
        let _ = update(&mut app, Message::SelectDown);
    }

    assert!(app.selected < 4);
}

#[test]
fn single_item_navigation_is_stable()
{
    let mut app = make_app_with_entries(vec![entry("Only")], 3, 10);

    let _ = update(&mut app, Message::SelectDown);
    let _ = update(&mut app, Message::SelectUp);
    let _ = update(&mut app, Message::SelectLeft);
    let _ = update(&mut app, Message::SelectRight);

    assert_eq!(app.selected, 0);
}

#[test]
fn large_grid_navigation()
{
    let entries: Vec<_> = (0..30).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 4, 30);

    app.selected = 3;

    let _ = update(&mut app, Message::SelectDown);
    assert_eq!(app.selected, 7);

    let _ = update(&mut app, Message::SelectUp);
    assert_eq!(app.selected, 3);
}

#[test]
fn zero_columns_does_not_break_navigation()
{
    let entries: Vec<_> = (0..5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 0, 5);

    let _ = update(&mut app, Message::SelectDown);
    assert!(app.selected < 5);
}
