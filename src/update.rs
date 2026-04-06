// ============ IMPORTS ============
use iced::Task;
use iced::widget::{Id, operation, scrollable};




// ============ CRATES ============
use crate::helpers::update_helpers::*;
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn update(app: &mut AppData, message: Message) -> Task<Message>
{
	match message
	{

                Message::KeyboardEvent(key_event) =>
                {
                    use iced::keyboard;
                    let msg = match key_event
                    {
                        keyboard::Event::KeyPressed { key, modifiers, .. } =>
                        {
                            // In shell mode: ArrowUp/Down cycle through command history
                            // instead of moving the selection cursor.
                            if app.shell_mode
                            {
                                use iced::keyboard::key::Named;
                                match &key
                                {
                                    iced::keyboard::Key::Named(Named::ArrowUp) =>
                                        Some(Message::ShellHistoryUp),
                                    iced::keyboard::Key::Named(Named::ArrowDown) =>
                                        Some(Message::ShellHistoryDown),
                                    _ => crate::subscription::handle_key_pressed(key, modifiers, &app.config.keybinds),
                                }
                            }
                            else
                            {
                                crate::subscription::handle_key_pressed(key, modifiers, &app.config.keybinds)
                            }
                        }
                        keyboard::Event::KeyReleased { key, .. } =>
                            crate::subscription::handle_key_released(key, &app.config.keybinds),
                        _ => None,
                    };
                    if let Some(inner) = msg
                    {
                        return update(app, inner);
                    }
                }

		Message::QueryChanged(query) =>
                {
			if app.alt_pressed
                        {
				return Task::none();
			}
			return on_query_changed(app, query);
		}

		Message::Scrolled(y, viewport_height, content_height) =>
                {
			app.scroll_offset = y;
			app.viewport_h = viewport_height;
			app.content_h = content_height;
		}

		Message::ScrollTo(offset) =>
		{
			// offset is an absolute pixel value; convert to relative [0,1].
			// Guard against uninitialised dimensions (before the first
			// Scrolled event arrives) so we don't snap to a wrong position.
			if app.content_h <= 0.0 || app.viewport_h <= 0.0 {
				return Task::none();
			}
			let scrollable_range = (app.content_h - app.viewport_h).max(1.0);
			let relative = (offset / scrollable_range).clamp(0.0, 1.0);
			app.scroll_offset = relative;
			return operation::snap_to(
				Id::new("results_scroll"),
				scrollable::RelativeOffset { x: 0.0, y: relative },
			);
		}

                Message::SelectUp    => return on_select_up(app),
                Message::SelectDown  => return on_select_down(app),
                Message::SelectLeft  => return on_select_left(app),
                Message::SelectRight => return on_select_right(app),
		Message::AltPressed(state) => app.alt_pressed = state,
		Message::EntriesLoaded(entries) => return on_entries_loaded(app, entries),
		Message::LaunchNth(index) => return on_launch_nth(app, index),
		Message::RelaunchLast => return on_relaunch_last(app),
		Message::CopyToClipboard(value) => return on_copy_to_clipboard(app, value),
		Message::CopiedFeedbackClear => app.copy_feedback = false,
		Message::Launch(exec) => return on_launch(app, exec),
		Message::ShellHistoryUp   => return crate::helpers::update_helpers::on_shell_history_up(app),
		Message::ShellHistoryDown => return crate::helpers::update_helpers::on_shell_history_down(app),
		Message::HoverEntry(index) =>
		{
			app.hovered = Some(index);
			// Sync keyboard selection to wherever the mouse is.
			let max = crate::helpers::update_helpers::visible_count(app);
			if index < max
			{
				app.selected = index;
			}
		}
		Message::HoverClear => app.hovered = None,
		Message::Close => return iced::exit(),
		_ => {}
	}
	Task::none()
}
