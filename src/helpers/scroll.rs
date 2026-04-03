// ============ IMPORTS ============
use iced::Task;
use iced::widget::{Id, scrollable, operation};




// ============ CRATES ============
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn scroll_to_selected(app: &mut AppData) -> Task<Message>
{
    let cfg     = &app.config;
    let ec      = &cfg.entry;
    let cols    = cfg.window.grid_side_items.max(1);
    let spacing = cfg.window.entry_spacing as f32;
    let max     = app.filtered.len().min(cfg.window.max_results);
    let total_rows = max.div_ceil(cols);
    if total_rows <= 1 { return Task::none(); }

    // We need real dimensions — bail until on_scroll has fired at least once
    if app.viewport_h <= 0.0 || app.content_h <= 0.0 { return Task::none(); }

    let visible_h        = app.viewport_h;
    let total_content_h  = app.content_h;
    let scrollable_range = (total_content_h - visible_h).max(0.0);
    if scrollable_range <= 0.0 { return Task::none(); }

    // Compute per-row heights the same way view.rs does —
    // a row is "tall" if any entry in it has a visible comment.
    let ep     = ec.padding;
    let base_h = (ec.name_size as f32) + ep[0] as f32 * 2.0 + 8.0;
    let tall_h = base_h + (ec.comment_size as f32) + 6.0;

    let mut row_tops: Vec<f32> = Vec::with_capacity(total_rows);
    let mut cursor = 0.0_f32;
    for r in 0..total_rows
    {
        row_tops.push(cursor);
        let start = r * cols;
        let end   = (start + cols).min(max);
        let row_has_comment = app.filtered[start..end].iter()
            .any(|e| ec.show_comment && !e.comment.is_empty());
        let row_h = if row_has_comment { tall_h } else { base_h };
        cursor += row_h + if r + 1 < total_rows { spacing } else { 0.0 };
    }
    // cursor is now the exact content height we gave iced
    let computed_content_h = cursor;

    let sel_row   = app.selected / cols;
    if sel_row >= row_tops.len() { return Task::none(); }
    let row_h_sel =
    {
        let start = sel_row * cols;
        let end   = (start + cols).min(max);
        let has_comment = app.filtered[start..end].iter()
            .any(|e| ec.show_comment && !e.comment.is_empty());
        if has_comment { tall_h } else { base_h }
    };

    // Map iced's relative offset back to pixels using the *actual* content height
    let actual_range    = (total_content_h - visible_h).max(0.0);
    let viewport_top_px = app.scroll_offset * actual_range;
    let viewport_bot_px = viewport_top_px + visible_h;

    let sel_top_px = row_tops[sel_row];
    let sel_bot_px = sel_top_px + row_h_sel;

    if sel_top_px >= viewport_top_px && sel_bot_px <= viewport_bot_px
    {
        return Task::none(); // already fully visible
    }

    let target_px = if sel_bot_px > viewport_bot_px
    {
        sel_bot_px - visible_h  // scroll down so bottom of row is visible
    }
    else
    {
        sel_top_px              // scroll up so top of row is visible
    };

    // Convert pixel target back to relative offset using the real scrollable range.
    // Scale by computed_content_h / total_content_h to correct for any iced rounding.
    let scale = if total_content_h > 0.0 { computed_content_h / total_content_h } else { 1.0 };
    let new_y = ((target_px * scale) / actual_range).clamp(0.0, 1.0);
    app.scroll_offset = new_y;

    operation::snap_to(
        Id::new("results_scroll"),
        scrollable::RelativeOffset { x: 0.0, y: new_y },
    )
}
