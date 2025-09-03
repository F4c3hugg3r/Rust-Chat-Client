use crate::UI::app::App;
use crate::UI::tabs::tabs::SelectedTab;
use Constraint::{Length, Min};
use color_eyre::owo_colors::OwoColorize;
use ratatui::prelude::Buffer;
use ratatui::prelude::Rect;
use ratatui::style::Modifier;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Tabs;
use ratatui::widgets::Wrap;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, Widget},
};
use strum::IntoEnumIterator;
use tui_textarea::TextArea;

/// Rendert die gesamte UI.
/// Diese Funktion nimmt ein &mut App, damit das TextArea Widget mutiert werden kann.
pub fn render_ui(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    let vertical = Layout::vertical([Length(1), Min(0)]);
    let [header_area, inner_area] = vertical.areas(area);

    let horizontal = Layout::horizontal(Constraint::from_percentages([70, 30]));
    let [tabs_area, title_area] = horizontal.areas(header_area);

    {
        let buf = frame.buffer_mut();
        render_title(title_area, buf);
        render_tabs(app, tabs_area, buf);
        // render_footer(footer_area, buf);
    }
    app.selected_tab.render(app, frame, inner_area);
}

fn render_tabs(app: &mut App, area: Rect, buf: &mut Buffer) {
    let titles = SelectedTab::iter().map(SelectedTab::title);
    let highlight_style = Style::default()
        .fg(app.selected_tab.palette().fg)
        .bg(app.selected_tab.palette().bg);
    let selected_tab_index = app.selected_tab as usize;
    Tabs::new(titles)
        .highlight_style(highlight_style)
        .select(selected_tab_index)
        .padding("", "")
        .divider(" ")
        .render(area, buf);
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new(Line::from("change tab with [Shift <] | [Shift >] ").dim())
        .alignment(Alignment::Right)
        .render(area, buf);
}

// fn render_footer(area: Rect, buf: &mut Buffer) {
//     Line::raw("◄ ► to change tab | Press q to quit")
//         .centered()
//         .render(area, buf);
// }

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
