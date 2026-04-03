// ============ IMPORTS ============
use iced::{Task, widget::{Id, operation, scrollable, operation::focus}};





// ============ CRATES ============
use crate::helpers::{launch::{filter_entries, launch_app}, scroll::scroll_to_selected};
use crate::{AppData, Message};





// ============ FUNCTIONS ============
pub fn update(app: &mut AppData, message: Message) -> Task<Message>
{
    match message
    {
        Message::EntriesLoaded(entries) =>
        {
            app.loading       = false;
            app.filtered      = entries.clone();
            app.entries       = entries;
            app.selected      = 0;
            app.scroll_offset = 0.0;
            return Task::batch(vec!
            [
                focus("search_input"),
                operation::snap_to(Id::new("results_scroll"), scrollable::RelativeOffset { x: 0.0, y: 0.0 })
            ]);
        }

        Message::QueryChanged(q) =>
        {
            app.query         = q.clone();
            app.selected      = 0;
            app.scroll_offset = 0.0;
            app.viewport_h    = 0.0;
            app.content_h     = 0.0;
            app.filtered      = filter_entries(&app.entries, &q, &app.config);
        }

        Message::Scrolled(y, vh, ch) =>
        {
            app.scroll_offset = y;
            app.viewport_h    = vh;
            app.content_h     = ch;
        }

        Message::SelectUp =>
        {
            if !app.filtered.is_empty()
            {
                let cols = app.config.window.grid_side_items.max(1);
                let max  = app.filtered.len().min(app.config.window.max_results);
                if max == 0 { return Task::none(); }
                let row_offset = app.selected / cols;
                let col_offset = app.selected % cols;
                let new_row = if row_offset == 0 
                { 
                    (max - 1) / cols 
                } 
                else 
                { 
                    row_offset - 1 
                };
                app.selected = (new_row * cols + col_offset).min(max - 1);
                return scroll_to_selected(app);
            }
        }

        Message::SelectDown =>
        {
            if !app.filtered.is_empty()
            {
                let cols = app.config.window.grid_side_items.max(1);
                let max  = app.filtered.len().min(app.config.window.max_results);
                if max == 0 { return Task::none(); }
                app.selected = (app.selected + cols) % max;
                return scroll_to_selected(app);
            }
        }

        Message::SelectLeft =>
        {
            if !app.filtered.is_empty()
            {
                let max = app.filtered.len().min(app.config.window.max_results).max(1);
                if max == 0 { return Task::none(); }
                app.selected = (app.selected + max - 1) % max;
                return scroll_to_selected(app);
            }
        }

        Message::SelectRight =>
        {
            if !app.filtered.is_empty()
            {
                let max = app.filtered.len().min(app.config.window.max_results).max(1);
                if max == 0 { return Task::none(); }
                app.selected = (app.selected + 1) % max;
                return scroll_to_selected(app);
            }
        }

        Message::CopyToClipboard(value) =>
        {
            app.copy_feedback = true;
            let secs = app.config.behaviour.copy_feedback_seconds;
            let copy = std::process::Command::new("wl-copy").arg(&value).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).output();
            drop(copy);
            return Task::perform
            (
                async move { tokio::time::sleep(std::time::Duration::from_secs_f32(secs)).await; },
                |_| Message::CopiedFeedbackClear
            );
        }

        Message::CopiedFeedbackClear =>
        {
            app.copy_feedback = false;
        }

        Message::Launch(exec) =>
        {
            if exec.is_empty()
            {
                if let Some(entry) = app.filtered.get(app.selected)
                {
                    let value = entry.name.trim_start_matches("= ").to_string();
                    return update(app, Message::CopyToClipboard(value));
                }
                return Task::none();
            }
            let terminal = app.filtered.iter().find(|e| e.exec == exec).map(|e| e.terminal).unwrap_or(false);
            launch_app(&exec, &app.config, terminal);
            if app.config.behaviour.close_on_launch { return iced::exit(); }
        }

        Message::Close => { return iced::exit(); }

        _ => {}
    }
    Task::none()
}
