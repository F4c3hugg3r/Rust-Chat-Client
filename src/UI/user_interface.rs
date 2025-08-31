use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, Widget},
};

use crate::UI::app::App;

/// Rendert die gesamte UI.
/// Diese Funktion nimmt ein &mut App, damit das TextArea Widget mutiert werden kann.
pub fn render_ui<'a>(app: &mut App<'a>, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(5)])
        .split(area);

    app.chat_size = chunks[0].as_size();

    // Nachrichtenbereich (oben)
    let message_block = Block::bordered()
        .title("Chat")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let chat_messages = if app.messages.is_empty() {
        "No messages yet. Press 'i' or Tab to start typing.".to_string()
    } else {
        app.messages.join("\n")
    };

    let message_field = Paragraph::new(format!("{}\n\n{}", app.title, chat_messages))
        .block(message_block)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .scroll((app.vertical_scroll as u16, 0));

    frame.render_widget(message_field, chunks[0]);

    // Scrollbar
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    );

    let style = Style::new().fg(Color::Cyan).bg(Color::Black);

    // Eingabebereich (unten)
    let input_block = Block::bordered()
        .title("Input")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(style);

    app.text_input.set_block(input_block);
    app.text_input.set_style(Style::new().bg(Color::Black));
    app.text_input.render(chunks[1], frame.buffer_mut());
}
