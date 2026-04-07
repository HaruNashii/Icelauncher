// ============ IMPORTS ============
use iced::widget::{Id, operation, scrollable};
use iced::Task;




// ============ CRATES ============
use crate::ron::{EntryConfig, IconConfig, LabelPosition};
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn scroll_to_selected(app: &mut AppData) -> Task<Message>
{
	let cols = app.config.window.grid_side_items.max(1);
	let max = app.filtered.len().min(app.config.window.max_results);
	let total_rows = max.div_ceil(cols);

	if total_rows <= 1 
        {
		return Task::none();
	}
	if app.viewport_h <= 0.0 || app.content_h <= 0.0 
        {
		return Task::none();
	}

	let row_heights = compute_row_heights(app, cols, max, total_rows);
	let row_tops = compute_row_tops(&row_heights, app.config.window.entry_spacing as f32);

	let sel_row = app.selected / cols;
	if sel_row >= row_tops.len() 
        {
		return Task::none();
	}

	let sel_row_top = row_tops[sel_row];
	let sel_row_height = row_heights[sel_row];
	let sel_row_bottom = sel_row_top + sel_row_height;

	let scrollable_range = (app.content_h - app.viewport_h).max(0.0);
	if scrollable_range <= 0.0 
        {
		return Task::none();
	}

	let viewport_top = app.scroll_offset * scrollable_range;
	let viewport_bottom = viewport_top + app.viewport_h;

	if sel_row_top >= viewport_top && sel_row_bottom <= viewport_bottom 
        {
		return Task::none();
	}

	let target_px = if sel_row_bottom > viewport_bottom 
        {
		sel_row_bottom - app.viewport_h
	} 
        else 
        {
		sel_row_top
	};

	let computed_content_h: f32 = row_tops.last().copied().unwrap_or(0.0) + row_heights.last().copied().unwrap_or(0.0);
	let scale = if app.content_h > 0.0 { computed_content_h / app.content_h } else { 1.0 };
	let new_offset = ((target_px * scale) / scrollable_range).clamp(0.0, 1.0);

	app.scroll_offset = new_offset;

	operation::snap_to(Id::new("results_scroll"), scrollable::RelativeOffset { x: 0.0, y: new_offset })
}


pub fn row_height(entry_config: &EntryConfig, icon_config: &IconConfig, has_comment: bool) -> f32
{
	let padding     = entry_config.padding;
	let text_height = (entry_config.name_size as f32) + if has_comment
	{
	    (entry_config.comment_size as f32) + (entry_config.name_comment_spacing as f32)
	}
	else 
        { 
            0.0 
        };

	match entry_config.label_position
	{
		// Icon stacked above or below label — total height is icon + gap + text + padding
		LabelPosition::Below | LabelPosition::Above =>
		{
			let icon_h = if icon_config.show { icon_config.height as f32 } else { 0.0 };
			let gap    = if icon_config.show { icon_config.gap    as f32 } else { 0.0 };
			(padding[0] as f32) + icon_h + gap + text_height + (padding[2] as f32)
		}
		// Icon beside label — height is whichever is taller + padding
		LabelPosition::Left | LabelPosition::Right =>
		{
			let icon_h = if icon_config.show { icon_config.height as f32 } else { 0.0 };
			(padding[0] as f32) + text_height.max(icon_h) + (padding[2] as f32) + 8.0
		}
	}
}


fn compute_row_heights(app: &AppData, cols: usize, max: usize, total_rows: usize) -> Vec<f32>
{
	let entry_config = &app.config.entry;
	let icon_config  = &app.config.icon;
	(0..total_rows)
		.map(|row| 
                {
			let start = row * cols;
			let end = (start + cols).min(max);
			let row_has_comment = app.filtered[start..end]
				.iter()
				.any(|e| entry_config.show_comment && !e.comment.is_empty());
			row_height(entry_config, icon_config, row_has_comment)
		})
		.collect()
}


fn compute_row_tops(row_heights: &[f32], spacing: f32) -> Vec<f32>
{
	let mut tops = Vec::with_capacity(row_heights.len());
	let mut cursor = 0.0_f32;

	for (i, &height) in row_heights.iter().enumerate() 
        {
		tops.push(cursor);
		cursor += height;
		if i + 1 < row_heights.len() 
                {
			cursor += spacing;
		}
	}

	tops
}
