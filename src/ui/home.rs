use crate::app::{App, InputKind, KeyMode};

use ratatui::{prelude::*, widgets::*};

pub fn render(app: &mut App, f: &mut Frame) {
    if let [title_layout, main_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Percentage(100)])
        .split(f.size())[..]
    {
        if let [key_layout, value_layout] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(main_layout)[..]
        {
            if let [key_pattern_layout, key_main_layout] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Percentage(100)])
                .split(key_layout)[..]
            {
                render_title(app, f, title_layout);

                render_key_pattern_input(app, f, key_pattern_layout);
                render_key_list(app, f, key_main_layout);

                render_value_para(app, f, value_layout);
            }
        }
    };
}

fn render_key_pattern_input(app: &mut App, f: &mut Frame, l: Rect) {
    let input = Paragraph::new(app.input_key_pattern.as_str())
        .style(match app.key_mode {
            KeyMode::Command => Style::default(),
            KeyMode::Input(InputKind::SearchPattern) => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Search Pattern"));
    f.render_widget(input, l);
}

fn render_title(app: &mut App, f: &mut Frame, l: Rect) {
    let is_conn = match app.get_current_redis_conn().is_conn() {
        true => "●",
        false => "○",
    };
    let title_para = Paragraph::new(format!(
        "  {}/{} {} {}",
        app.current_url_index + 1,
        app.get_redis_urls_len(),
        app.get_current_redis_url(),
        is_conn
    ))
    .style(Style::default().fg(Color::LightYellow))
    .alignment(Alignment::Left);

    f.render_widget(title_para, l);
}
fn render_key_list(app: &mut App, f: &mut Frame, l: Rect) {
    let current_redis_conn = app.get_current_redis_conn();
    let items: Vec<ListItem> = current_redis_conn
        .open_kvs
        .iter()
        .map(|s| ListItem::new(s.get_key_title()))
        .collect();
    let mut state = ListState::default().with_selected(Some(current_redis_conn.current_key_index));
    let list = List::new(items)
        .block(
            Block::default()
                .border_set(symbols::border::PLAIN)
                .title("Keys")
                .borders(Borders::all()),
        )
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_stateful_widget(list, l, &mut state);
}
fn render_value_para(app: &mut App, f: &mut Frame, l: Rect) {
    let current_redis_conn = app.get_current_redis_conn();
    let value = current_redis_conn.get_curret_value();
    let para = Paragraph::new(value)
        .block(
            Block::default()
                .title("Value")
                .title_alignment(Alignment::Left)
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Left);
    f.render_widget(para, l);
}
