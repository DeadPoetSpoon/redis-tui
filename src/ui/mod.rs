
pub mod about;
pub mod home;

use crate::app::{App,CurrentScreen};
use ratatui::Frame;


pub fn render(app: &mut App, f: &mut Frame){
    match app.current_screen {
        CurrentScreen::Home => home::render(app, f),
        CurrentScreen::About => about::render(f),
    }
}