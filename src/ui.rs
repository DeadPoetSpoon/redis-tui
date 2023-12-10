use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame){
    let para = Paragraph::new(format!("
    Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    urls: {} \n
    ",
    app.get_redis_url_string()))
    .block(
        Block::default()
        .title("Redis TUI")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center);
    f.render_widget(para, f.size());
}