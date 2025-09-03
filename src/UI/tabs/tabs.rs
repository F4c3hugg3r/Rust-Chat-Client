use color_eyre::owo_colors::OwoColorize;
use ratatui::Frame;
use ratatui::prelude::Buffer;
use ratatui::prelude::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Widget;
use ratatui::{style::Color, text::Line, widgets::Paragraph};
use strum_macros::{Display, EnumIter, FromRepr};

use crate::UI::app::App;
use crate::UI::tabs::{chat, users};
use crate::types::BLUE_COLOR;
use crate::types::TURKIS_COLOR;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, Debug)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Chat")]
    Chat = 0,
    #[strum(to_string = "Users")]
    Users = 1,
    #[strum(to_string = "Help")]
    Help = 2,
    #[strum(to_string = "Tab 4")]
    Tab4 = 3,
}

impl SelectedTab {
    pub fn render(self, app: &mut App, frame: &mut Frame, area: Rect) {
        // in a real app these might be separate widgets
        match self {
            Self::Chat => self.render_tab0(app, frame, area),
            Self::Users => self.render_tab1(app, frame, area),
            Self::Help => self.render_tab2(app, frame, area),
            Self::Tab4 => self.render_tab3(app, frame, area),
        }
    }
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn title(self) -> Line<'static> {
        Line::from(format!("  {self}  "))
    }

    fn render_tab0(self, app: &mut App, frame: &mut Frame, area: Rect) {
        chat::render_chat_tab(app, frame, area);
    }

    fn render_tab1(self, app: &mut App, frame: &mut Frame, area: Rect) {
        users::UsersTable::render_users_tab(app, frame, area);
    }

    fn render_tab2(self, app: &mut App, frame: &mut Frame, area: Rect) {}

    fn render_tab3(self, app: &mut App, frame: &mut Frame, area: Rect) {}

    pub const fn palette(self) -> Palette {
        match self {
            Self::Users => Palette {
                fg: BLUE_COLOR, // blue
                bg: Color::Black,
                border: BLUE_COLOR,
            },
            Self::Tab4 => Palette {
                fg: Color::Indexed(63), // purple
                bg: Color::Black,
                border: Color::Indexed(63),
            },
            Self::Chat => Palette {
                fg: TURKIS_COLOR, // turkis
                bg: Color::Black,
                border: TURKIS_COLOR,
            },
            Self::Help => Palette {
                fg: Color::Rgb(191, 53, 53), // orange
                bg: Color::Black,
                border: Color::Rgb(191, 53, 53),
            },
        }
    }
}

pub struct Palette {
    pub fg: Color,
    pub bg: Color,
    pub border: Color,
}
