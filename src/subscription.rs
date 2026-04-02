// ============ IMPORTS ============
use iced::{event, keyboard::{self, Key, key::Named}};




// ============ CRATES ============
use crate::helpers::load_apps_stream;
use crate::{AppData, Message};




// ============ FUNCTIONS ============
pub fn subscription(_app: &AppData) -> iced::Subscription<Message>
{
    let key_sub = event::listen_with(|event, _status, _id|
    {
        match event
        {
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key
            {
                Key::Named(Named::Escape)    => Some(Message::Close),
                Key::Named(Named::ArrowUp)   => Some(Message::SelectUp),
                Key::Named(Named::ArrowDown) => Some(Message::SelectDown),
                _ => None,
            },
            _ => None,
        }
    });

    let load_sub = iced::Subscription::run(load_apps_stream);
    iced::Subscription::batch([key_sub, load_sub])
}
