// ============ IMPORTS ============
use iced::widget::{Id, operation, operation::focus, scrollable};
use iced::Task;




// ============ CRATES ============
use crate::helpers::filter::filter_entries;
use crate::helpers::launch::launch_app;
use crate::helpers::scroll::scroll_to_selected;
use crate::{update, AppData, AppEntry, Message};




// ============ FUNCTIONS ============
pub fn on_entries_loaded(app: &mut AppData, entries: Vec<AppEntry>) -> Task<Message>
{
	app.loading = false;
	app.filtered = filter_entries(&entries, "", &app.config, &app.frecency);
	app.entries = entries;
	app.selected = 0;
	app.scroll_offset = 0.0;

	Task::batch(vec![
		focus("search_input"),
		operation::snap_to(
			Id::new("results_scroll"),
			scrollable::RelativeOffset { x: 0.0, y: 0.0 },
		),
	])
}


pub fn on_query_changed(app: &mut AppData, query: String) -> Task<Message>
{
	app.filtered = filter_entries(&app.entries, &query, &app.config, &app.frecency);
	app.query = query;
	app.selected = 0;
	// Any manual query change breaks out of history cycling.
	app.shell_history_index = None;
	app.scroll_offset = 0.0;
	app.viewport_h = 0.0;
	app.content_h = 0.0;

	// Reset the scrollable widget's actual position, not just our cached state.
	operation::snap_to(
		Id::new("results_scroll"),
		scrollable::RelativeOffset { x: 0.0, y: 0.0 },
	)
}


pub fn on_select_up(app: &mut AppData) -> Task<Message>
{
	if app.filtered.is_empty() {
		return Task::none();
	}

	let cols = app.config.window.grid_side_items.max(1);
	let max = visible_count(app);
	let current_row = app.selected / cols;
	let current_col = app.selected % cols;
	let prev_row = if current_row == 0 { (max - 1) / cols } else { current_row - 1 };

	app.selected = (prev_row * cols + current_col).min(max - 1);
	scroll_to_selected(app)
}


pub fn on_select_down(app: &mut AppData) -> Task<Message>
{
	if app.filtered.is_empty() {
		return Task::none();
	}

	let cols = app.config.window.grid_side_items.max(1);
	let max = visible_count(app);

	if max == 0 {
		return Task::none();
	}

	app.selected = (app.selected + cols) % max;
	scroll_to_selected(app)
}


pub fn on_select_left(app: &mut AppData) -> Task<Message>
{
	if app.filtered.is_empty() {
		return Task::none();
	}

	let max = visible_count(app).max(1);
	app.selected = (app.selected + max - 1) % max;
	scroll_to_selected(app)
}


pub fn on_select_right(app: &mut AppData) -> Task<Message>
{
	if app.filtered.is_empty() {
		return Task::none();
	}

	let max = visible_count(app).max(1);
	app.selected = (app.selected + 1) % max;
	scroll_to_selected(app)
}


pub fn on_launch_nth(app: &mut AppData, index: usize) -> Task<Message>
{
	let max = visible_count(app);
	if index >= max {
		return Task::none();
	}

	let entry = app.filtered[index].clone();
	if entry.exec.is_empty() {
		let value = calc_display_value(&entry.name);
		return update(app, Message::CopyToClipboard(value));
	}

	record_and_launch(app, &entry.exec);
	exit_if_configured(app)
}


pub fn on_relaunch_last(app: &mut AppData) -> Task<Message>
{
	let Some(exec) = app.last_launched.clone() else { return Task::none() };
	record_and_launch(app, &exec);
	exit_if_configured(app)
}


pub fn on_copy_to_clipboard(app: &mut AppData, value: String) -> Task<Message>
{
	app.copy_feedback = true;

	let delay_secs = app.config.behaviour.copy_feedback_seconds;

	if app.wl_copy_available {
		let _ = std::process::Command::new("wl-copy")
			.arg(&value)
			.stdin(std::process::Stdio::null())
			.stdout(std::process::Stdio::null())
			.stderr(std::process::Stdio::null())
			.spawn();
	}

	Task::perform(
		async move { tokio::time::sleep(std::time::Duration::from_secs_f32(delay_secs)).await },
		|_| Message::CopiedFeedbackClear,
	)
}


pub fn on_launch(app: &mut AppData, exec: String) -> Task<Message>
{
	if exec.is_empty()
	{
		if app.shell_mode
		{
			// User hit Enter with no selection — run whatever they typed as-is.
			let query = app.query.trim().to_string();
			if query.is_empty() { return Task::none(); }
			record_and_launch(app, &query);
			return exit_if_configured(app);
		}
		let Some(entry) = app.filtered.get(app.selected) else { return Task::none() };
		let value = calc_display_value(&entry.name);
		return update(app, Message::CopyToClipboard(value));
	}

	record_and_launch(app, &exec);
	exit_if_configured(app)
}


pub fn record_and_launch(app: &mut AppData, exec: &str)
{
	app.frecency.record_in_memory(exec);
	app.last_launched = Some(exec.to_string());

	// Persist the updated frecency data in the background.
	// We already called record_in_memory above, so just clone the current
	// state and save it — do NOT call save_record (which would call
	// record_in_memory a second time, double-counting this launch).
	let store = app.frecency.clone();
	tokio::spawn(async move {
		tokio::task::spawn_blocking(move || store.save()).await.ok();
	});

	// In shell mode every entry is a raw command — never wrap with a terminal prefix
	// unless the user typed something that matches a known terminal=true .desktop entry.
	let is_terminal = if app.shell_mode
	{
		false
	}
	else
	{
		app.filtered.iter().find(|e| e.exec == exec).map(|e| e.terminal).unwrap_or(false)
	};

	launch_app(exec, &app.config, is_terminal);
}


pub fn exit_if_configured(app: &AppData) -> Task<Message>
{
	if app.config.behaviour.close_on_launch { iced::exit() } else { Task::none() }
}


pub fn visible_count(app: &AppData) -> usize
{
	app.filtered.len().min(app.config.window.max_results)
}


pub fn calc_display_value(name: &str) -> String
{
	name.trim_start_matches("= ").to_string()
}


pub fn on_shell_history_up(app: &mut AppData) -> Task<Message>
{
	if !app.shell_mode {
		return Task::none();
	}

	// Build history list lazily: top frecency entries, most-recent first.
	let history = app.frecency.top_n(50);
	if history.is_empty() {
		return Task::none();
	}

	let next_index = match app.shell_history_index {
		None    => 0,
		Some(i) => (i + 1).min(history.len() - 1),
	};

	app.shell_history_index = Some(next_index);
	let query = history[next_index].clone();
	on_query_changed(app, query)
}


pub fn on_shell_history_down(app: &mut AppData) -> Task<Message>
{
	if !app.shell_mode {
		return Task::none();
	}

	match app.shell_history_index {
		None | Some(0) => {
			app.shell_history_index = None;
			on_query_changed(app, String::new())
		}
		Some(i) => {
			let history = app.frecency.top_n(50);
			let prev_index = i - 1;
			app.shell_history_index = Some(prev_index);
			let query = history.get(prev_index).cloned().unwrap_or_default();
			on_query_changed(app, query)
		}
	}
}
