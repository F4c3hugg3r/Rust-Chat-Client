use color_eyre::Result;
use crossterm::event::KeyModifiers;
use itertools::Itertools;
use rand::Fill;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
};
use serde::{Deserialize, Serialize};
use style::palette::tailwind;

use crate::types::{self, GROUP_HELP_FLAG, HELP_FLAG};
use crate::{UI::app::App, types::JsonClient};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HelpItem {
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug)]
pub struct FillState {
    help: bool,
    group_help: bool,
}

impl FillState {
    pub fn new() -> FillState {
        FillState {
            help: false,
            group_help: false,
        }
    }
}

// TODO default wert für Help Item
#[derive(Debug)]
pub struct HelpTable {
    pub fill_state: FillState,
    pub state: TableState,
    items: Vec<HelpItem>,
    longest_item_lens: (u16, u16),
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

const INFO_TEXT: [&str; 1] = ["/help & /users & /group users are moved to the tabs"];

const ITEM_HEIGHT: usize = 4;

#[derive(Debug)]
pub struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    pub const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

impl HelpItem {
    const fn ref_array(&self) -> [&String; 2] {
        [&self.command, &self.description]
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn command(&self) -> &str {
        &self.command
    }
}

impl HelpTable {
    pub fn new() -> Self {
        let data_vec = Vec::from([HelpItem {
            command: String::from("/register {name}"),
            description: String::from("registeres a client"),
        }]);
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len().saturating_sub(1)) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
            fill_state: FillState::new(),
        }
    }

    pub fn filled(&self) -> bool {
        if self.fill_state.help && self.fill_state.group_help {
            return true;
        }
        false
    }

    pub fn push_items(&mut self, items: Vec<HelpItem>, identifier: &str) {
        if self.filled() {
            return;
        }
        match identifier {
            HELP_FLAG => {
                if !self.fill_state.help {
                    self.items.remove(0);
                    self.fill_state.help = true;
                    for item in items {
                        self.items.push(item);
                    }
                }
            }
            GROUP_HELP_FLAG => {
                if !self.fill_state.group_help {
                    self.fill_state.group_help = true;
                    for item in items {
                        self.items.push(item);
                    }
                }
            }
            _ => {}
        }
        self.longest_item_lens = constraint_len_calculator(&self.items);
    }

    pub fn remove_items(&mut self) {
        self.items = Vec::default();
        self.fill_state = FillState::new();
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }

    pub fn render_help_tab(app: &mut App, frame: &mut Frame, area: Rect) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(area);

        app.help_table.set_colors();

        app.help_table.render_table(frame, rects[0]);
        app.help_table.render_scrollbar(frame, rects[0]);
        app.help_table.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = ["Command", "Description"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
}

fn constraint_len_calculator(items: &[HelpItem]) -> (u16, u16) {
    let name_len = items
        .iter()
        .map(HelpItem::command)
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0);
    let address_len = items
        .iter()
        .map(HelpItem::description)
        .flat_map(str::lines)
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, address_len as u16)
}
