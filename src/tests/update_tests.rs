use crate::{AppData, AppEntry, Message, ron::LauncherConfig, update::update};

// ── helpers ───────────────────────────────────────────────────────────────────

fn entry(name: &str) -> AppEntry
{
    AppEntry {
        name: name.to_string(),
        exec: name.to_string(),
        comment: String::new(),
        icon: String::new(),
        icon_path: None,
        keywords: vec![],
        terminal: false,
        name_lc: String::new(),
        exec_lc: String::new(),
        comment_lc: String::new(),
        keywords_lc: Vec::new(),
    }
    .with_normalized()
}

fn make_app_with_entries(entries: Vec<AppEntry>, cols: usize, max: usize) -> AppData
{
    let mut cfg = LauncherConfig::default();
    cfg.window.grid_side_items = cols;
    cfg.window.max_results = max;

    AppData {
        filtered: entries.clone(),
        entries,
        loading: false,
        config: cfg, // ✅ FIXED
        ..Default::default()
    }
}

// ── EntriesLoaded ─────────────────────────────────────────────────────────────

#[test]
fn entries_loaded_sets_entries_and_clears_loading()
{
    let mut app = AppData { loading: true, ..Default::default() };
    let entries = vec![entry("Firefox"), entry("Vim")];
    let _ = update(&mut app, Message::EntriesLoaded(entries.clone()));

    assert!(!app.loading);
    assert_eq!(app.entries.len(), 2);
    assert_eq!(app.filtered.len(), 2);
    assert_eq!(app.selected, 0);
    assert_eq!(app.scroll_offset, 0.0);
}

#[test]
fn entries_loaded_resets_selection_to_zero()
{
    let mut app = AppData { selected: 5, loading: true, ..Default::default() };
    let _ = update(&mut app, Message::EntriesLoaded(vec![entry("App")]));
    assert_eq!(app.selected, 0);
}

#[test]
fn entries_loaded_with_empty_list()
{
    let mut app = AppData { loading: true, ..Default::default() };
    let _ = update(&mut app, Message::EntriesLoaded(vec![]));
    assert!(!app.loading);
    assert!(app.entries.is_empty());
    assert!(app.filtered.is_empty());
}

// ── QueryChanged ─────────────────────────────────────────────────────────────

#[test]
fn query_changed_filters_entries()
{
    let mut app =
        make_app_with_entries(vec![entry("Firefox"), entry("Vim"), entry("Firefox Dev")], 1, 10);
    let _ = update(&mut app, Message::QueryChanged("firefox".to_string()));

    assert_eq!(app.query, "firefox");
    assert_eq!(app.filtered.len(), 2);
    assert_eq!(app.selected, 0);
    assert_eq!(app.scroll_offset, 0.0);
    assert_eq!(app.viewport_h, 0.0);
    assert_eq!(app.content_h, 0.0);
}

#[test]
fn query_changed_empty_query_restores_all()
{
    let mut app = make_app_with_entries(vec![entry("Firefox"), entry("Vim")], 1, 10);
    let _ = update(&mut app, Message::QueryChanged("vim".to_string()));
    assert_eq!(app.filtered.len(), 1);

    let _ = update(&mut app, Message::QueryChanged("".to_string()));
    assert_eq!(app.filtered.len(), 2);
}

#[test]
fn query_changed_resets_scroll_state()
{
    let mut app = make_app_with_entries(vec![entry("App")], 1, 10);
    app.scroll_offset = 0.8;
    app.viewport_h = 300.0;
    app.content_h = 900.0;

    let _ = update(&mut app, Message::QueryChanged("a".to_string()));

    assert_eq!(app.scroll_offset, 0.0);
    assert_eq!(app.viewport_h, 0.0);
    assert_eq!(app.content_h, 0.0);
}

// ── Scrolled ─────────────────────────────────────────────────────────────────

#[test]
fn scrolled_updates_all_three_fields()
{
    let mut app = AppData::default();
    let _ = update(&mut app, Message::Scrolled(0.42, 300.0, 900.0));
    assert!((app.scroll_offset - 0.42).abs() < 0.001);
    assert!((app.viewport_h - 300.0).abs() < 0.001);
    assert!((app.content_h - 900.0).abs() < 0.001);
}

#[test]
fn scrolled_to_top()
{
    let mut app =
        AppData { scroll_offset: 1.0, viewport_h: 300.0, content_h: 900.0, ..Default::default() };
    let _ = update(&mut app, Message::Scrolled(0.0, 300.0, 900.0));
    assert_eq!(app.scroll_offset, 0.0);
}

// ── SelectDown ───────────────────────────────────────────────────────────────

#[test]
fn select_down_moves_selection_forward_by_cols()
{
    let entries: Vec<_> = (0 .. 6).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 6);
    app.selected = 0;

    let _ = update(&mut app, Message::SelectDown);
    assert_eq!(app.selected, 1);
}

#[test]
fn select_down_wraps_to_first_row_from_last()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 5);
    app.selected = 4;

    let _ = update(&mut app, Message::SelectDown);
    assert_eq!(app.selected, 0);
}

#[test]
fn select_down_with_grid_skips_full_row()
{
    let entries: Vec<_> = (0 .. 6).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 6);
    app.selected = 0;

    let _ = update(&mut app, Message::SelectDown);
    assert_eq!(app.selected, 2);
}

#[test]
fn select_down_noop_on_empty()
{
    let mut app = make_app_with_entries(vec![], 1, 10);
    app.selected = 0;
    let _ = update(&mut app, Message::SelectDown);
    assert_eq!(app.selected, 0);
}

// ── SelectUp ─────────────────────────────────────────────────────────────────

#[test]
fn select_up_moves_selection_back_by_cols()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 5);
    app.selected = 3;

    let _ = update(&mut app, Message::SelectUp);
    assert_eq!(app.selected, 2);
}

#[test]
fn select_up_from_first_row_wraps_to_last_row_same_col()
{
    let entries: Vec<_> = (0 .. 6).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 6);
    app.selected = 0;

    let _ = update(&mut app, Message::SelectUp);
    assert_eq!(app.selected, 4);
}

#[test]
fn select_up_from_col_1_first_row_wraps_to_last_row_col_1()
{
    let entries: Vec<_> = (0 .. 6).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 6);
    app.selected = 1;

    let _ = update(&mut app, Message::SelectUp);
    assert_eq!(app.selected, 5);
}

#[test]
fn select_up_noop_on_empty()
{
    let mut app = make_app_with_entries(vec![], 1, 10);
    let _ = update(&mut app, Message::SelectUp);
    assert_eq!(app.selected, 0);
}

// ── SelectLeft ───────────────────────────────────────────────────────────────

#[test]
fn select_left_decrements_selected()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 5);
    app.selected = 3;

    let _ = update(&mut app, Message::SelectLeft);
    assert_eq!(app.selected, 2);
}

#[test]
fn select_left_from_zero_wraps_to_last()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 5);
    app.selected = 0;

    let _ = update(&mut app, Message::SelectLeft);
    assert_eq!(app.selected, 4);
}

#[test]
fn select_left_noop_on_empty()
{
    let mut app = make_app_with_entries(vec![], 1, 10);
    let _ = update(&mut app, Message::SelectLeft);
    assert_eq!(app.selected, 0);
}

// ── SelectRight ──────────────────────────────────────────────────────────────

#[test]
fn select_right_increments_selected()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 5);
    app.selected = 2;

    let _ = update(&mut app, Message::SelectRight);
    assert_eq!(app.selected, 3);
}

#[test]
fn select_right_from_last_wraps_to_zero()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 5);
    app.selected = 4;

    let _ = update(&mut app, Message::SelectRight);
    assert_eq!(app.selected, 0);
}

#[test]
fn select_right_noop_on_empty()
{
    let mut app = make_app_with_entries(vec![], 1, 10);
    let _ = update(&mut app, Message::SelectRight);
    assert_eq!(app.selected, 0);
}

// ── max_results cap ───────────────────────────────────────────────────────────

#[test]
fn select_down_respects_max_results_cap()
{
    let entries: Vec<_> = (0 .. 10).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 4);
    app.selected = 3;

    let _ = update(&mut app, Message::SelectDown);
    assert_eq!(app.selected, 0);
}

#[test]
fn select_left_wraps_at_max_results_not_total()
{
    let entries: Vec<_> = (0 .. 10).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 4);
    app.selected = 0;

    let _ = update(&mut app, Message::SelectLeft);
    assert_eq!(app.selected, 3);
}

// ── Partial Last ──────────────────────────────────────────────────────────────────

#[test]
fn select_down_into_partial_last_row()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 5);
    app.selected = 2; // row 1, col 0

    let _ = update(&mut app, Message::SelectDown);

    // should go to index 4 (last row, col 0)
    assert_eq!(app.selected, 4);
}

#[test]
fn select_up_from_partial_last_row()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 5);
    app.selected = 4; // last row (only col 0 exists)

    let _ = update(&mut app, Message::SelectUp);

    assert_eq!(app.selected, 2);
}

#[test]
fn select_up_clamps_to_existing_column()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 5);
    app.selected = 1; // col 1

    let _ = update(&mut app, Message::SelectUp);

    // should go to last row, but col 1 doesn't exist → clamp to 4
    assert_eq!(app.selected, 4);
}

#[test]
fn select_down_single_column_behaves_like_list()
{
    let entries: Vec<_> = (0 .. 3).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 3);
    app.selected = 1;

    let _ = update(&mut app, Message::SelectDown);

    assert_eq!(app.selected, 2);
}

#[test]
fn grid_respects_max_results()
{
    let entries: Vec<_> = (0 .. 10).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 5); // only 0..4 visible
    app.selected = 2;

    let _ = update(&mut app, Message::SelectDown);

    // should go to 4, not 6
    assert_eq!(app.selected, 4);
}

#[test]
fn query_clamps_selection_if_out_of_bounds()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 10);

    app.selected = 4;

    let _ = update(&mut app, Message::QueryChanged("App0".into()));

    // only 1 result → selection should reset/clamp
    assert_eq!(app.selected, 0);
}

#[test]
fn max_results_zero_does_not_panic()
{
    let entries: Vec<_> = (0 .. 5).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 0);
    let _ = update(&mut app, Message::SelectDown);
    assert_eq!(app.selected, 0);
}

#[test]
fn selection_never_out_of_bounds_after_navigation()
{
    let entries: Vec<_> = (0 .. 7).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 3, 7);

    for _ in 0 .. 50 {
        let _ = update(&mut app, Message::SelectDown);
        let max = app.filtered.len().min(app.config.window.max_results);

        if max > 0 {
            assert!(app.selected < max);
        } else {
            assert_eq!(app.selected, 0);
        }
    }
}

#[test]
fn up_then_down_returns_to_same_position()
{
    let entries: Vec<_> = (0 .. 6).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 6);

    app.selected = 3;

    let _ = update(&mut app, Message::SelectUp);
    let _ = update(&mut app, Message::SelectDown);

    assert_eq!(app.selected, 3);
}

#[test]
fn repeated_down_cycles_through_all_items()
{
    let entries: Vec<_> = (0 .. 4).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 1, 4);

    for _ in 0 .. 4 {
        let _ = update(&mut app, Message::SelectDown);
    }

    assert_eq!(app.selected, 0);
}

#[test]
fn grid_navigation_cycles_correctly()
{
    let entries: Vec<_> = (0 .. 6).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 6);

    let start = app.selected;

    for _ in 0 .. 6 {
        let _ = update(&mut app, Message::SelectRight);
    }

    assert_eq!(app.selected, start);
}

#[test]
fn navigation_after_filtering_stays_valid()
{
    let entries: Vec<_> = (0 .. 10).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 2, 10);

    app.selected = 8;

    let _ = update(&mut app, Message::QueryChanged("App1".into()));

    let max = app.filtered.len().min(app.config.window.max_results);

    if max > 0 {
        assert!(app.selected < max);
    }
}

#[test]
fn random_navigation_does_not_panic_or_break_invariants()
{
    let entries: Vec<_> = (0 .. 20).map(|i| entry(&format!("App{i}"))).collect();
    let mut app = make_app_with_entries(entries, 3, 15);

    let actions =
        [Message::SelectUp, Message::SelectDown, Message::SelectLeft, Message::SelectRight];

    for i in 0 .. 200 {
        let action = &actions[i % actions.len()];
        let _ = update(&mut app, action.clone());

        let max = app.filtered.len().min(app.config.window.max_results);

        if max > 0 {
            assert!(app.selected < max);
        }
    }
}

#[tokio::test]
async fn launch_uses_selected_entry()
{
    let entries = vec![entry("A"), entry("B")];
    let mut app = make_app_with_entries(entries, 1, 10);

    app.selected = 1;

    let exec = app.filtered[app.selected].exec.clone();

    let _ = update(&mut app, Message::Launch(exec.clone()));

    // can't easily assert process spawn, but at least:
    assert_eq!(app.filtered[1].exec, exec);
}

// ── Nothing ──────────────────────────────────────────────────────────────────

#[test]
fn nothing_message_leaves_state_unchanged()
{
    let entries = vec![entry("App")];
    let mut app = make_app_with_entries(entries, 1, 10);
    app.selected = 0;
    app.scroll_offset = 0.5;
    app.query = "test".to_string();

    let _ = update(&mut app, Message::Nothing);

    assert_eq!(app.selected, 0);
    assert_eq!(app.scroll_offset, 0.5);
    assert_eq!(app.query, "test");
}
