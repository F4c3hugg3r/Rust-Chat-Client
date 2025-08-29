use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};
use tui_textarea::TextArea;

use crate::UI::app::App;

/// Rendert die gesamte UI.
/// Diese Funktion nimmt ein &mut App, damit das TextArea Widget mutiert werden kann.
pub fn render_ui<'a>(app: &mut App<'a>, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(5)])
        .split(area);

    // Nachrichtenbereich (oben)
    let message_block = Block::bordered()
        .title("event-driven-async-generated")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let messages_text = if app.messages.is_empty() {
        "No messages yet. Press 'i' or Tab to start typing.".to_string()
    } else {
        app.messages.join("\n\n")
    };

    let header_text = format!(
        "Insert Title here.\n\
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press left and right to increment and decrement the counter respectively.\n\
        Counter: {}\n",
        app.counter,
    );

    let message_field = Paragraph::new(format!("{}\n\n{}", header_text, messages_text))
        .block(message_block)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();

    message_field.render(chunks[0], buf);

    // Eingabebereich (unten)
    let input_block = Block::bordered()
        .title("Input")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    app.text_input.set_block(input_block);
    app.text_input.render(chunks[1], buf);
}
