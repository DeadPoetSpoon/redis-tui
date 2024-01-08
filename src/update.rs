use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, CurrentScreen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    let current_redis_conn = app.get_current_redis_conn();
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }else {
                let _ = current_redis_conn.conn();
                current_redis_conn.get_keys("*".to_string());
            }
        },
        KeyCode::Char('j')=>current_redis_conn.switch_to_next_key(),
        KeyCode::Char('k')=>current_redis_conn.switch_to_before_key(),
        KeyCode::Enter=>current_redis_conn.fetch_curret_value(),
        KeyCode::Tab=>{app.current_screen = CurrentScreen::About},
        _ => {}
    };
    
}