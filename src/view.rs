// ============ IMPORTS ============
use iced::Element;




// ============ CRATES ============
use crate::helpers::layout::{assemble_layout, build_window_panel};
use crate::helpers::results::build_results_block;
use crate::helpers::search_bar::build_search_bar;
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn view(app: &AppData) -> Element<'_, Message>
{
	let search_bar = build_search_bar(app);
	let results_block = build_results_block(app);
	let content = assemble_layout(app, search_bar, results_block);
	build_window_panel(app, content)
}
