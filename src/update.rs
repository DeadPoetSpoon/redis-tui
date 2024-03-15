use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputKind, KeyMode};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.key_mode {
        KeyMode::Command => {
            match key_event.code {
                KeyCode::Char('q') => app.quit(),
                KeyCode::Char('c') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit()
                    } else {
                        let _ = app.conn_current();
                    }
                }
                KeyCode::Char('j') => app.switch_to_next_key(),
                KeyCode::Char('k') => app.switch_to_before_key(),
                KeyCode::Char('h') => app.switch_to_before_conn(),
                KeyCode::Char('l') => app.switch_to_next_conn(),
                KeyCode::Char('s') => app.key_mode = KeyMode::Input(InputKind::SearchPattern),
                KeyCode::Delete => app.delete_current_key(),
                KeyCode::Char('r') => app.delete_current_key(),
                KeyCode::Enter => app.fetch_curret_value(),
                KeyCode::Tab => app.switch_to_next_screen(),
                _ => {}
            };
        }
        KeyMode::Input(InputKind::SearchPattern) => match key_event.code {
            KeyCode::Char(x) => {
                app.input_key_pattern.push(x);
            }
            KeyCode::Backspace => {
                app.input_key_pattern.pop();
            }
            KeyCode::Delete => {
                app.input_key_pattern.clear();
            }
            KeyCode::Esc => {
                app.key_mode = KeyMode::Command;
            }
            KeyCode::Enter => {
                app.key_mode = KeyMode::Command;
                app.get_current_keys();
            }
            _ => {}
        },
    };
}
