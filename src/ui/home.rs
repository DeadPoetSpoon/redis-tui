use crate::app::App;

use ratatui::{prelude::*, widgets::*};

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Percentage(100)])
        .split(f.size());
    let sub_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(layout[1]);

    render_title(app, f, layout[0]);

    render_key_list(app, f, sub_layout[0]);


    render_value_para(app, f, sub_layout[1]);
}

fn render_title(app: &mut App, f: &mut Frame, l: Rect) {
    let title_para = Paragraph::new(format!(
        "  {}/{} {}",
        app.current_url_index + 1,
        app.get_redis_urls_len(),
        app.get_current_redis_url()
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
        .map(|s| ListItem::new(s.key.to_string()))
        .collect();
    let mut state = ListState::default().with_selected(Some(current_redis_conn.current_key_index));
    let list = List::new(items)
        .block(
            Block::default()
                .border_set(symbols::border::PLAIN)
                .title("Keys")
                .borders(Borders::all()),
        )
        .style(Style::default().fg(Color::Black))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_stateful_widget(list, l, &mut state);
}
fn render_value_para(app: &mut App, f: &mut Frame, l: Rect){
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
    .style(Style::default().fg(Color::LightGreen))
    .alignment(Alignment::Left);
    f.render_widget(para, l);
}
