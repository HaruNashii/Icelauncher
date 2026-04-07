use crate::{AppData, AppEntry, ron::LauncherConfig};

// ── helpers ──────────────────────────────────────────────────────────────────

fn entry_plain(name: &str) -> AppEntry
{
    AppEntry 
    {
        name: name.to_string(),
        exec: name.to_string(),
        ..Default::default()
    }
}

fn entry_with_comment(name: &str) -> AppEntry
{
    AppEntry 
    {
        name: name.to_string(),
        exec: name.to_string(),
        comment: "A description that makes the row taller".to_string(),
        ..Default::default()
    }
}

fn make_app(
    entries: Vec<AppEntry>,
    selected: usize,
    cols: usize,
    max: usize,
    viewport_h: f32,
    content_h: f32,
    offset: f32,
) -> AppData
{
    let mut cfg = LauncherConfig::default();
    cfg.window.grid_side_items = cols;
    cfg.window.max_results = max;
    cfg.window.entry_spacing = 3;
    cfg.entry.name_size = 14;
    cfg.entry.comment_size = 11;
    cfg.entry.padding = [6, 10, 0, 0];
    cfg.entry.show_comment = true;
    cfg.icon.show = false; // disable icons so row_height() matches test expectations (28px base)

    AppData {
        filtered: entries.clone(),
        entries,
        selected,
        config: cfg,
        viewport_h,
        content_h,
        scroll_offset: offset,
        ..Default::default()
    }
}

fn row_heights() -> (f32, f32)
{
    let name_size = 14_f32;
    let comment_size = 11_f32;
    // padding = [6, 10, 0, 0]  →  top=6, bottom=0
    // row_height uses padding[0]+padding[2], not padding[0]*2
    let pad_top = 6_f32;
    let pad_bottom = 0_f32;
    // name_comment_spacing defaults to 2 (not hardcoded 6)
    let name_comment_spacing = 2_f32;
    let base_h = name_size + pad_top + pad_bottom + 8.0; // 14+6+0+8 = 28
    let tall_h = base_h + comment_size + name_comment_spacing; // 28+11+2 = 41
    (base_h, tall_h)
}

// ── no-scroll cases ───────────────────────────────────────────────────────────

#[test]
fn returns_none_task_when_zero_viewport()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0 .. 10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let mut app = make_app(entries, 5, 1, 10, 0.0, 500.0, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, 0.0);
}

#[test]
fn returns_none_task_when_zero_content_h()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0 .. 10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let mut app = make_app(entries, 5, 1, 10, 300.0, 0.0, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, 0.0);
}

#[test]
fn no_scroll_when_single_row()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries = vec![entry_plain("OnlyApp")];
    let mut app = make_app(entries, 0, 1, 1, 200.0, 34.0, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, 0.0);
}

#[test]
fn no_scroll_when_content_fits_in_viewport()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0 .. 3).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 3.0 + spacing * 2.0;
    let mut app = make_app(entries, 2, 1, 3, content, content, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, 0.0);
}

// ── scroll down ───────────────────────────────────────────────────────────────

#[test]
fn scrolls_down_when_selected_below_viewport()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0 .. 10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 10.0 + spacing * 9.0;
    let viewport = base_h * 3.0 + spacing * 2.0;

    let mut app = make_app(entries, 9, 1, 10, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);

    assert!(app.scroll_offset > 0.0, "expected scroll down, got {}", app.scroll_offset);
    assert!(app.scroll_offset <= 1.0);
}

#[test]
fn scroll_offset_clamped_to_one()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0 .. 20).map(|i| entry_plain(&format!("App{i}"))).collect();
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
    let entries: Vec<AppEntry> = (0 .. 10).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 10.0 + spacing * 9.0;
    let viewport = base_h * 3.0;

    let mut app = make_app(entries, 0, 1, 10, viewport, content, 1.0);
    let _ = scroll_to_selected(&mut app);

    assert!(app.scroll_offset < 1.0, "expected scroll up, got {}", app.scroll_offset);
    assert!(app.scroll_offset < 0.1);
}

// ── already-visible item ──────────────────────────────────────────────────────

#[test]
fn no_scroll_when_item_already_visible()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0 .. 5).map(|i| entry_plain(&format!("App{i}"))).collect();
    let (base_h, _) = row_heights();
    let spacing = 3.0;
    let content = base_h * 5.0 + spacing * 4.0;
    let viewport = content - base_h;

    let mut app = make_app(entries, 2, 1, 5, viewport, content, 0.0);
    let initial = app.scroll_offset;
    let _ = scroll_to_selected(&mut app);
    assert_eq!(app.scroll_offset, initial);
}

// ── grid (multi-column) ───────────────────────────────────────────────────────

#[test]
fn grid_two_cols_selected_row_computed_correctly()
{
    use crate::helpers::scroll::scroll_to_selected;
    let entries: Vec<AppEntry> = (0 .. 6).map(|i| entry_plain(&format!("App{i}"))).collect();
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
    let entries = vec![entry_with_comment("App0"), entry_plain("App1"), entry_plain("App2")];
    let (base_h, tall_h) = row_heights();
    let spacing = 3.0;
    let content = tall_h + spacing + base_h + spacing + base_h;
    let viewport = tall_h; // viewport = first row height

    let mut app = make_app(entries, 2, 1, 3, viewport, content, 0.0);
    let _ = scroll_to_selected(&mut app);
    assert!(app.scroll_offset > 0.0);
    assert!(app.scroll_offset <= 1.0);
}
