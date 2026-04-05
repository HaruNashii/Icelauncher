// ============ IMPORTS ============
use iced::event;
use iced::keyboard::{self, Key, Modifiers, key::Named};




// ============ CRATES ============
use crate::helpers::desktop::load_apps_stream;
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn subscription(_app: &AppData) -> iced::Subscription<Message>
{
	iced::Subscription::batch([
		event::listen_with(handle_keyboard_event),
		iced::Subscription::run(load_apps_stream),
	])
}


fn handle_keyboard_event(
	event: iced::Event,
	_status: iced::event::Status,
	_id: iced::window::Id,
) -> Option<Message>
{
	let iced::Event::Keyboard(key_event) = event else { return None };

	match key_event
	{
		keyboard::Event::KeyPressed { key, modifiers, .. } => handle_key_pressed(key, modifiers),
		keyboard::Event::KeyReleased { key, .. } => handle_key_released(key),
		_ => None,
	}
}


fn handle_key_pressed(key: Key, modifiers: Modifiers) -> Option<Message>
{
	if key == Key::Named(Named::Alt) {
		return Some(Message::AltPressed(true));
	}

	if modifiers.contains(Modifiers::ALT) {
		return handle_alt_shortcut(key);
	}

	handle_navigation_key(key)
}


fn handle_key_released(key: Key) -> Option<Message>
{
	if key == Key::Named(Named::Alt) {
		return Some(Message::AltPressed(false));
	}
	None
}


fn handle_alt_shortcut(key: Key) -> Option<Message>
{
	let Key::Character(c) = key else { return None };

	let digit = c.chars().next().and_then(|ch| ch.to_digit(10))?;
	if (1..=9).contains(&digit) {
		return Some(Message::LaunchNth((digit - 1) as usize));
	}

	match c.as_str()
	{
		"l" | "L" => Some(Message::RelaunchLast),
		_ => None,
	}
}


fn handle_navigation_key(key: Key) -> Option<Message>
{
	match key
	{
		Key::Named(Named::Escape) => Some(Message::Close),
		Key::Named(Named::ArrowUp) => Some(Message::SelectUp),
		Key::Named(Named::ArrowDown) => Some(Message::SelectDown),
		Key::Named(Named::ArrowLeft) => Some(Message::SelectLeft),
		Key::Named(Named::ArrowRight) | Key::Named(Named::Tab) => Some(Message::SelectRight),
		_ => None,
	}
}
