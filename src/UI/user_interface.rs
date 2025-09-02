use crate::UI::app::App;
use color_eyre::owo_colors::OwoColorize;
use ratatui::style::Modifier;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Wrap;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, Widget},
};
use tui_textarea::TextArea;

/// Rendert die gesamte UI.
/// Diese Funktion nimmt ein &mut App, damit das TextArea Widget mutiert werden kann.
pub fn render_ui(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(5)])
        .split(area);

    app.chat_size = chunks[0].as_size();

    let style = Style::new().fg(Color::Cyan).bg(Color::Black);

    // Nachrichtenbereich (oben)
    let message_block = Block::bordered()
        .title(format!(" {} ", app.title.clone()))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(style);

    let message_field = Paragraph::new(app.messages.clone())
        .block(message_block)
        .bg(Color::Black)
        .scroll((app.vertical_scroll as u16, 0))
        .wrap(Wrap { trim: true });

    frame.render_widget(message_field, chunks[0]);

    // Scrollbar
    app.vertical_scroll_state = app.vertical_scroll_state.content_length(app.messages.len());
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[0],
        &mut app.vertical_scroll_state,
    );

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

// Einfache Farbstile
pub fn red_span(text: String) -> Span<'static> {
    Span::styled(text, Style::new().fg(Color::Rgb(191, 53, 53)))
}

pub fn blue_span(text: String) -> Span<'static> {
    Span::styled(text, Style::new().fg(Color::Rgb(53, 113, 191)))
}

pub fn purple_span(text: String) -> Span<'static> {
    Span::styled(text, Style::new().fg(Color::Indexed(63)))
}

pub fn turkis_span(text: String) -> Span<'static> {
    Span::styled(text, Style::new().fg(Color::Rgb(53, 191, 188)))
}

pub fn green_span(text: String) -> Span<'static> {
    Span::styled(text, Style::new().fg(Color::Rgb(62, 138, 41)))
}

// Faint (blass)
pub fn faint_span(text: String) -> Span<'static> {
    Span::styled(text, Style::new().add_modifier(Modifier::DIM))
}
