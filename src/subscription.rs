// ============ IMPORTS ============
use iced::event;
use iced::keyboard::{Key, Modifiers, key::Named};




// ============ CRATES ============
use crate::helpers::desktop::{load_apps_stream, load_shell_commands_stream};
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn subscription(app: &AppData) -> iced::Subscription<Message>
{
	let stream = if app.shell_mode
	{
		iced::Subscription::run(load_shell_commands_stream)
	}
	else
	{
		iced::Subscription::run(load_apps_stream)
	};

	iced::Subscription::batch
	([
		event::listen_with(|event, _status, _id| match event
		{
			iced::Event::Keyboard(k) => Some(Message::KeyboardEvent(k)),
			_ => None,
		}),
		stream,
	])
}


pub fn handle_key_pressed(key: Key, modifiers: Modifiers, keybinds: &crate::ron::KeybindConfig) -> Option<Message>
{
	// Track modifier-key press
	if is_named_key(&key, &keybinds.launch_alt_prefix)
	{
		return Some(Message::AltPressed(true));
	}

	// Modifier-held shortcuts
	if modifier_active(modifiers, &keybinds.launch_alt_prefix)
	{
		return handle_alt_shortcut(key, keybinds);
	}

	handle_navigation_key(key, keybinds)
}


pub fn handle_key_released(key: Key, keybinds: &crate::ron::KeybindConfig) -> Option<Message>
{
	if is_named_key(&key, &keybinds.launch_alt_prefix)
	{
		return Some(Message::AltPressed(false));
	}
	None
}


fn handle_alt_shortcut(key: Key, keybinds: &crate::ron::KeybindConfig) -> Option<Message>
{
	let Key::Character(c) = key else { return None };

	// Alt+1-9 quick-launch: only consume the key if it IS a digit in 1..=9.
	// Do not use `?` here — a non-digit key must fall through to the relaunch
	// check below, not cause an early None return.
	if let Some(digit) = c.chars().next().and_then(|ch| ch.to_digit(10)) && (1..=9).contains(&digit)
	{
		return Some(Message::LaunchNth((digit - 1) as usize));
	}

	// Configurable relaunch key (default "l")
	let ch = c.chars().next()?.to_lowercase().to_string();
	let rk = keybinds.relaunch_key.to_lowercase();
	if ch == rk
	{
		return Some(Message::RelaunchLast);
	}

	None
}


fn handle_navigation_key(key: Key, keybinds: &crate::ron::KeybindConfig) -> Option<Message>
{
	let key_name = key_to_string(&key);

	if keybinds.close.iter().any(|k| k == &key_name)        { return Some(Message::Close);       }
	if keybinds.select_up.iter().any(|k| k == &key_name)    { return Some(Message::SelectUp);    }
	if keybinds.select_down.iter().any(|k| k == &key_name)  { return Some(Message::SelectDown);  }
	if keybinds.select_left.iter().any(|k| k == &key_name)  { return Some(Message::SelectLeft);  }
	if keybinds.select_right.iter().any(|k| k == &key_name) { return Some(Message::SelectRight); }

	None
}


/// Convert an iced `Key` to the string representation used in keybind config.
fn key_to_string(key: &Key) -> String
{
	match key
	{
		Key::Named(Named::Escape)    => "Escape".into(),
		Key::Named(Named::ArrowUp)   => "ArrowUp".into(),
		Key::Named(Named::ArrowDown) => "ArrowDown".into(),
		Key::Named(Named::ArrowLeft) => "ArrowLeft".into(),
		Key::Named(Named::ArrowRight)=> "ArrowRight".into(),
		Key::Named(Named::Tab)       => "Tab".into(),
		Key::Named(Named::Enter)     => "Enter".into(),
		Key::Named(Named::Space)     => "Space".into(),
		Key::Named(Named::Backspace) => "Backspace".into(),
		Key::Character(c)            => c.to_string(),
		_                            => String::new(),
	}
}


/// Return true if `name` matches the modifier label in the keybind config (e.g. "Alt").
fn modifier_active(modifiers: Modifiers, name: &str) -> bool
{
	match name.to_lowercase().as_str()
	{
		"alt"   => modifiers.contains(Modifiers::ALT),
		"ctrl"  => modifiers.contains(Modifiers::CTRL),
		"shift" => modifiers.contains(Modifiers::SHIFT),
		"super" | "logo" => modifiers.contains(Modifiers::LOGO),
		_ => false,
	}
}


/// Return true if `key` is the named modifier key itself (so we can track its press/release).
fn is_named_key(key: &Key, name: &str) -> bool
{
	match name.to_lowercase().as_str()
	{
		"alt"   => *key == Key::Named(Named::Alt),
		"ctrl"  => *key == Key::Named(Named::Control),
		"shift" => *key == Key::Named(Named::Shift),
		"super" | "logo" => *key == Key::Named(Named::Super),
		_ => false,
	}
}
