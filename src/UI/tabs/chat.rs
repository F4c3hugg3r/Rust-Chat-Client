use crate::UI::app::App;
use crate::UI::user_interface::blue_span;
use crate::types;
use color_eyre::owo_colors::OwoColorize;
use ratatui::prelude::Rect;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::Wrap;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, Widget},
};

pub fn render_chat_tab(app: &mut App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(5)])
        .split(area);
    app.chat_size = chunks[0].as_size();

    let style = Style::new().fg(types::DARK_TURKIS_COLOR).bg(Color::Black);

    // Nachrichtenbereich (oben)
    let message_block = Block::bordered()
        .title(format!(" {} ", app.title.clone()))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(style);

    let message_field = Paragraph::new(app.messages.lines.clone())
        .block(message_block)
        .bg(Color::Black)
        .scroll((app.vertical_scroll as u16, 0))
        .wrap(Wrap { trim: true });

    frame.render_widget(message_field, chunks[0]);

    // Scrollbar
    app.vertical_scroll_state = app
        .vertical_scroll_state
        .content_length(app.messages.lines.len());
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[0],
        &mut app.vertical_scroll_state,
    );

    // Eingabebereich (unten)
    let input_block = Block::bordered()
        .title(" Previous Input [Shift ←] | Last Input [Shift →]")
        .title_alignment(Alignment::Right)
        .border_type(BorderType::Rounded)
        .border_style(style);

    app.text_input.set_cursor_line_style(Style::default());
    app.text_input.set_block(input_block);
    app.text_input.set_style(Style::new().bg(Color::Black));
    app.text_input.render(chunks[1], frame.buffer_mut());
}
