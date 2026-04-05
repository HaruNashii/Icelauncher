// ============ IMPORTS ============
use iced::Task;




// ============ CRATES ============
use crate::helpers::update_helpers::*;
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn update(app: &mut AppData, message: Message) -> Task<Message>
{
	match message
	{
		Message::AltPressed(state) => {
			app.alt_pressed = state;
		}

		Message::EntriesLoaded(entries) => {
			return on_entries_loaded(app, entries);
		}

		Message::QueryChanged(query) => {
			if app.alt_pressed {
				return Task::none();
			}
			on_query_changed(app, query);
		}

		Message::Scrolled(y, viewport_height, content_height) => {
			app.scroll_offset = y;
			app.viewport_h = viewport_height;
			app.content_h = content_height;
		}

		Message::SelectUp    => return on_select_up(app),
		Message::SelectDown  => return on_select_down(app),
		Message::SelectLeft  => return on_select_left(app),
		Message::SelectRight => return on_select_right(app),

		Message::LaunchNth(index) => {
			return on_launch_nth(app, index);
		}

		Message::RelaunchLast => {
			return on_relaunch_last(app);
		}

		Message::CopyToClipboard(value) => {
			return on_copy_to_clipboard(app, value);
		}

		Message::CopiedFeedbackClear => {
			app.copy_feedback = false;
		}

		Message::Launch(exec) => {
			return on_launch(app, exec);
		}

		Message::Close => {
			return iced::exit();
		}

		_ => {}
	}
	Task::none()
}


