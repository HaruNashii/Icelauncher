// ============ IMPORTS ============
use iced::{Task, widget::operation::focus};




// ============ CRATES ============
use crate::{AppData, Message};
use crate::helpers::{filter_entries, launch_app};




// ============ FUNCTIONS ============
pub fn update(app: &mut AppData, message: Message) -> Task<Message>
{
    match message
    {
        Message::EntriesLoaded(entries) =>
        {
            app.loading  = false;
            app.filtered = entries.clone();
            app.entries  = entries;
            app.selected = 0;
            return focus("search_input");
        }

        Message::QueryChanged(q) =>
        {
            app.query    = q.clone();
            app.selected = 0;
            app.filtered = filter_entries(&app.entries, &q, &app.config);
        }

        Message::SelectUp =>
        {
            if !app.filtered.is_empty()
            {
                let cols = app.config.window.grid_side_items.max(1);
                let max  = app.filtered.len().min(app.config.window.max_results);
                if app.selected < cols
                {
                    // Already on the first row — wrap to last row, same column
                    let col      = app.selected % cols;
                    let last_row = (max - 1) / cols;
                    let target   = last_row * cols + col;
                    app.selected = target.min(max - 1);
                }
                else { app.selected -= cols; }
            }
        }
        
        Message::SelectDown =>
        {
            if !app.filtered.is_empty()
            {
                let cols = app.config.window.grid_side_items.max(1);
                let max  = app.filtered.len().min(app.config.window.max_results);
                let next = app.selected + cols;
                if next >= max
                {
                    app.selected %= cols;
                }
                else { app.selected = next; }
            }
        }

        Message::SelectLeft =>
        {
            if !app.filtered.is_empty()
            {
                if app.selected == 0 { app.selected = app.filtered.len().min(app.config.window.max_results) - 1; }
                else { app.selected -= 1; }
            }
        }
        
        Message::SelectRight =>
        {
            if !app.filtered.is_empty()
            {
                let max = app.filtered.len().min(app.config.window.max_results) - 1;
                if app.selected >= max { app.selected = 0; }
                else { app.selected += 1; }
            }
        }

        Message::Launch(exec) =>
        {
            let terminal = app.filtered.iter().find(|e| e.exec == exec).map(|e| e.terminal).unwrap_or(false);
            launch_app(&exec, &app.config, terminal);
            if app.config.behaviour.close_on_launch { std::process::exit(0); }
        }

        Message::Close => { std::process::exit(0); }

        _ => {}
    }
    Task::none()
}
