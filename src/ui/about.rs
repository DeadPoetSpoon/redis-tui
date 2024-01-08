use ratatui::{
    prelude::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame) {
    let para = Paragraph::new(
        "\n\nToy TUI for Redis\n\n\
    Home Page: https://github.com/DeadPoetSpoon/redis-tui\n\n\n\
    Redis TUI is licensed under the MIT License"
            .to_string(),
    )
    .block(
        Block::default()
            .title("Redis TUI")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::LightGreen))
    .alignment(Alignment::Center);
    f.render_widget(para, f.size());
}
