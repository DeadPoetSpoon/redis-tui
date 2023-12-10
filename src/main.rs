pub mod args;
pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;
use clap::Parser;
fn main() -> Result<()> {
    let app_args = args::AppArgs::parse();
    let mut app = App::new(&app_args);
    color_eyre::install()?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;
    tui.draw(&mut app)?;
    while !app.should_quit {
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => {
                update(&mut app, key_event);
                tui.draw(&mut app)?;
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
    tui.exit()?;
    Ok(())
}
