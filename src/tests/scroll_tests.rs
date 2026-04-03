use crate::{AppData, AppEntry};
use crate::ron::LauncherConfig;

// ── helpers ──────────────────────────────────────────────────────────────────

fn entry_plain(name: &str) -> AppEntry
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

fn entry_with_comment(name: &str) -> AppEntry
{
    AppEntry
    {
        name:      name.to_string(),
        exec:      name.to_string(),
        comment:   "A description that makes the row taller".to_string(),
        icon:      String::new(),
        icon_path: None,
        keywords:  vec![],
        terminal:  false,
    }
}

/// Build an AppData ready for scroll testing.
/// viewport_h / content_h must be supplied to simulate iced having reported real dimensions.
fn make_app(
    entries:    Vec<AppEntry>,
    selected:   usize,
    cols:       usize,
    max:        usize,
    viewport_h: f32,
    content_h:  f32,
    offset:     f32,
) -> AppData
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

/// Compute the expected base/tall row heights from config defaults used in make_app.
fn row_heights() -> (f32, f32)
{
    let name_size    = 14_f32;
    let comment_size = 11_f32;
    let pad_v        = 6_f32;
    let base_h = name_size + pad_v * 2.0 + 8.0;          // 14+12+8 = 34
    let tall_h = base_h + comment_size + 6.0;             // 34+11+6 = 51
    (base_h, tall_h)
}

// ── no-scroll cases ───────────────────────────────────────────────────────────

#[test]
fn returns_none_task_when_zero_viewport()
{
    // viewport_h = 0 → bail immediately
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0..10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let mut app = make_app(entries, 5, 1, 10, 0.0, 500.0, 0.0);
    // Should return Task::none() (which we can't inspect directly, but it mustn't panic)
    let _ = scroll_to_selected(&mut app);
    // scroll_offset unchanged
    assert_eq!(app.scroll_offset, 0.0);
}

#[test]
fn returns_none_task_when_zero_content_h()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0..10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let mut app = make_app(entries, 5, 1, 10, 300.0, 0.0, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, 0.0);
}

#[test]
fn no_scroll_when_single_row()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries = vec![entry_plain("OnlyApp")];
    // single row → total_rows = 1 → early return
    let mut app = make_app(entries, 0, 1, 1, 200.0, 34.0, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, 0.0);
}

#[test]
fn no_scroll_when_content_fits_in_viewport()
{
    use crate::helpers::scroll::scroll_to_selected;
    // 3 rows, big viewport → everything visible → no scroll needed
    let entries: Vec<AppEntry> = (0..3).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 3.0 + spacing * 2.0;
    // viewport = content → scrollable_range = 0 → bail
    let mut app = make_app(entries, 2, 1, 3, content, content, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, 0.0);
}

// ── scroll down ───────────────────────────────────────────────────────────────

#[test]
fn scrolls_down_when_selected_below_viewport()
{
    use crate::helpers::scroll::scroll_to_selected;
    // 10 rows, viewport shows ~3 rows, selected = row 9 (last)
    let entries: Vec<AppEntry> = (0..10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing     = 3.0;
    let content     = base_h * 10.0 + spacing * 9.0;
    let viewport    = base_h * 3.0 + spacing * 2.0;

    let mut app = make_app(entries, 9, 1, 10, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);

    // offset should be > 0 (scrolled down)
    assert!(app.scroll_offset > 0.0, "expected scroll down, got {}", app.scroll_offset);
    assert!(app.scroll_offset <= 1.0);
}

#[test]
fn scroll_offset_clamped_to_one()
{
    use crate::helpers::scroll::scroll_to_selected;
    // Pathological: selected = last, tiny viewport
    let entries: Vec<AppEntry> = (0..20).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 20.0 + spacing * 19.0;
    let viewport = base_h; // only 1 row visible

    let mut app = make_app(entries, 19, 1, 20, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert!(app.scroll_offset <= 1.0);
}

// ── scroll up ────────────────────────────────────────────────────────────────

#[test]
fn scrolls_up_when_selected_above_viewport()
{
    use crate::helpers::scroll::scroll_to_selected;
    // Start at bottom (offset=1.0), select first row → should scroll up
    let entries: Vec<AppEntry> = (0..10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing  = 3.0;
    let content  = base_h * 10.0 + spacing * 9.0;
    let viewport = base_h * 3.0;

    let mut app = make_app(entries, 0, 1, 10, viewport, content, 1.0);
    let _ = scroll_to_selected(&mut app);

    assert!(app.scroll_offset < 1.0, "expected scroll up, got {}", app.scroll_offset);
    // First row selected → should be fully at the top → offset near 0
    assert!(app.scroll_offset < 0.1);
}

// ── already-visible item ──────────────────────────────────────────────────────

#[test]
fn no_scroll_when_item_already_visible()
{
    use crate::helpers::scroll::scroll_to_selected;
    // 5 rows, viewport shows all 5, select row 2 → already visible
    let entries: Vec<AppEntry> = (0..5).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 5.0 + spacing * 4.0;
    // viewport slightly smaller so there IS a scrollable range
    let viewport = content - base_h;

    let mut app = make_app(entries, 2, 1, 5, viewport, content, 0.0);
    let initial = app.scroll_offset;
    let _ = scroll_to_selected(&mut app);
    // Row 2 is in the middle, should be visible from offset=0 with this viewport
    assert_eq!(app.scroll_offset, initial);
}

// ── grid (multi-column) ───────────────────────────────────────────────────────

#[test]
fn grid_two_cols_selected_row_computed_correctly()
{
    use crate::helpers::scroll::scroll_to_selected;
    // 6 entries, 2 cols → 3 rows. Select entry 4 (row 2, last row).
    let entries: Vec<AppEntry> = (0..6).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 3.0 + spacing * 2.0;
    let viewport = base_h; // only 1 row visible

    let mut app = make_app(entries, 4, 2, 6, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert!(app.scroll_offset > 0.0, "should have scrolled to show row 2");
}

// ── mixed-height rows ─────────────────────────────────────────────────────────

#[test]
fn tall_row_with_comment_accounted_in_scroll()
{
    use crate::helpers::scroll::scroll_to_selected;
    // Row 0: plain (base_h). Row 1: has comment (tall_h). Select row 1.
    let entries = vec![entry_plain("App0"), entry_with_comment("App1")];
    let (base_h, tall_h) = row_heights();
    let spacing = 3.0;
    let content = base_h + spacing + tall_h;
    let viewport = base_h; // only first row visible

    let mut app = make_app(entries, 1, 1, 2, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert!(app.scroll_offset > 0.0, "should scroll down to show tall row");
}

#[test]
fn plain_row_after_tall_row_positions_correctly()
{
    use crate::helpers::scroll::scroll_to_selected;
    // Row 0: comment (tall_h). Row 1: plain (base_h). Row 2: plain. Select row 2 from top.
    let entries = vec![
        entry_with_comment("App0"),
        entry_plain("App1"),
        entry_plain("App2"),
    ];
    let (base_h, tall_h) = row_heights();
    let spacing = 3.0;
    let content = tall_h + spacing + base_h + spacing + base_h;
    let viewport = tall_h; // viewport = first row height

    let mut app = make_app(entries, 2, 1, 3, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);
    // Must scroll down past the tall row + second row
    assert!(app.scroll_offset > 0.0);
    assert!(app.scroll_offset <= 1.0);
}
