use std::sync::Arc;

use crate::service::user_service::UserService;
use crate::types::*;
use crate::{
    UI::event::{AppEvent, Event, EventHandler},
    UI::user_interface,
    types::Response,
};
use crate::{
    UI::{
        input_history::InputHistory,
        user_interface::{blue_span, purple_span, red_span, turkis_span},
    },
    helper::lines_from_string,
};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use ratatui::{layout::Size, text::Text};
use tokio::sync::mpsc::Receiver;
use tui_textarea::{CursorMove, TextArea};

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    pub messages: Text<'a>,
    pub title: Line<'a>,
    pub user_service: Arc<UserService>,
    pub text_input: TextArea<'static>,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub chat_size: Size,
    pub history: InputHistory,
}

// TODO input history
// TODO tabelle?
// TODO webrtc?
impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(user_service: Arc<UserService>, mut receiver: Receiver<Response>) -> Self {
        let mut app = Self {
            running: true,
            events: EventHandler::new(),
            messages: Text::from(String::new()),
            user_service: user_service.clone(),
            text_input: TextArea::default(),
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            chat_size: Size::new(0, 0),
            title: Line::raw(DEFAULT_TITLE),
            history: InputHistory {
                current: -1,
                first: false,
                inputs: Vec::new(),
            },
        };

        tokio::spawn(async move {
            user_service.chat_client.clone().response_poller().await;
        });

        let sender = app.events.get_sender_clone();
        tokio::spawn(async move {
            while let Some(rsp) = receiver.recv().await {
                // FIXME add error handling if needed
                let _ = sender.send(Event::App(AppEvent::Response(rsp)));
            }
        });

        app
    }

    // Event parsing
    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| user_interface::render_ui(&mut self, frame))?;

            if self.text_input.is_empty() {
                self.history.save_input(String::new());
            }
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_events(key_event)?;
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::Enter => self.handle_message().await,
                    AppEvent::Response(response) => self.handle_response(response).await,
                },
            }
        }
        Ok(())
    }

    // sendet AppEvents
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        // TODO custom keymaps

        if key_event.modifiers == KeyModifiers::SHIFT {
            match key_event.code {
                KeyCode::Left | KeyCode::Right => {
                    let str = self.search_input_history(key_event.code);
                    self.text_input = TextArea::default();
                    self.text_input.insert_str(str);
                }
                _ => {
                    self.text_input.input(key_event);
                }
            }
            return Ok(());
        }

        match key_event.code {
            KeyCode::Esc => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Down => {
                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            KeyCode::Up => {
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            KeyCode::Enter => self.events.send(AppEvent::Enter),
            // Other handlers you could add here.
            _ => {
                self.text_input.input(key_event);
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn handle_response(&mut self, rsp: Response) {
        let lines = self.evaluate_response(rsp).await.unwrap_or_default();
        if !lines.is_empty() {
            self.display_message(lines);
        }
    }

    // TODO vec lines returnen, damit multi line output möglich ist
    pub async fn evaluate_response(&mut self, rsp: Response) -> Option<Vec<Line<'static>>> {
        match rsp {
            // error output
            Response { err, .. } if !err.is_empty() => {
                if err == IGNORE_RESPONSE_TAG {
                    return None;
                }
                Some(vec![Line::from(red_span(err.to_string()))])
            }

            // empty output
            Response { content, .. } if content.is_empty() => None,

            // register output
            Response { content, .. } if content == REGISTER_FLAG => {
                let client_name = self
                    .user_service
                    .chat_client
                    .client_name
                    .lock()
                    .await
                    .clone();
                self.switch_title(REGISTER_FLAG, [client_name, String::from("")]);
                Some(vec![
                    Line::from(blue_span(REGISTER_OUTPUT.to_string())),
                    Line::from(blue_span(REGISTER_HELP_OUTPUT.to_string())),
                    Line::from(blue_span(REGISTER_QUIT_OUTPUT.to_string())),
                ])
            }

            // server output
            Response { rsp_name, .. } if rsp_name.is_empty() => {
                // unregister output
                if rsp.content == UNREGISTER_FLAG {
                    self.switch_title(UNREGISTER_FLAG, [String::new(), String::new()]);
                }
                Some(vec![Line::from(blue_span(rsp.content))])
            }

            // one user left output
            Response { rsp_name, .. } if rsp_name == USER_REMOVE_FLAG => {
                Some(vec![Line::from(vec![
                    purple_span(rsp.content),
                    blue_span(String::from(" hat den Chat verlassen")),
                ])])
            }

            // one user joined output
            Response { rsp_name, .. } if rsp_name == USER_ADD_FLAG => Some(vec![Line::from(vec![
                purple_span(rsp.content),
                blue_span(String::from(" ist dem Chat beigetreten")),
            ])]),

            // add group output
            Response { rsp_name, .. } if rsp_name == ADD_GROUP_FLAG => {
                // TODO HandleAddGroup method hinzufügen
                let group_name = String::from("{insert group name}");
                let client_name = self
                    .user_service
                    .chat_client
                    .client_name
                    .lock()
                    .await
                    .clone();
                self.switch_title(ADD_GROUP_FLAG, [client_name, group_name.clone()]);
                Some(vec![
                    Line::from(vec![
                        blue_span("-> Du bist nun Teil der Gruppe ".to_string()),
                        turkis_span(group_name),
                    ]),
                    Line::from(blue_span(
                        " [ Private Nachrichten kannst du weiterhin außerhalb verschicken ]"
                            .to_string(),
                    )),
                ])
            }

            // leave group output
            Response { rsp_name, .. } if rsp_name == LEAVE_GROUP_FLAG => {
                // TODO logic
                let client_name = self
                    .user_service
                    .chat_client
                    .client_name
                    .lock()
                    .await
                    .clone();
                self.switch_title(REGISTER_FLAG, [client_name, String::new()]);
                Some(vec![
                    Line::from(blue_span(REGISTER_OUTPUT.to_string())),
                    Line::from(blue_span(REGISTER_HELP_OUTPUT.to_string())),
                    Line::from(blue_span(REGISTER_QUIT_OUTPUT.to_string())),
                ])
            }

            // slice output
            // TODO maybe als Tabs und Table

            // response output
            _ => {
                let mut result = vec![Line::from(vec![
                    turkis_span(rsp.rsp_name),
                    Span::from(": "),
                ])];
                result.extend(lines_from_string(&rsp.content));
                Some(result)
            }
        }
    }

    pub fn display_message(&mut self, lines: Vec<Line<'a>>) {
        for line in lines {
            self.messages.lines.push(line);
        }
    }

    pub async fn handle_message(&mut self) {
        let input = self.text_input.lines().join("\n");
        self.user_service.executor(input.as_str()).await;
        self.history.save_input(input);
        self.text_input = TextArea::default();
        self.scroll();
    }

    pub fn scroll(&mut self) {
        if self.messages.lines.len() > self.chat_size.height.saturating_sub(5).into() {
            self.vertical_scroll = self
                .messages
                .lines
                .len()
                .saturating_sub(self.chat_size.height.into());
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        }
    }

    pub fn switch_title(&mut self, title: &str, strings: [String; 2]) {
        match title {
            UNREGISTER_FLAG => self.title = Line::from(DEFAULT_TITLE),
            REGISTER_FLAG => {
                self.title = Line::from(vec![
                    Span::raw("Du bist registriert "),
                    purple_span(strings[0].clone()),
                    Span::raw("!"),
                ]);
            }
            ADD_GROUP_FLAG => {
                self.title = Line::from(vec![
                    purple_span(strings[0].clone()),
                    Span::raw(", du bist in der Gruppe "),
                    purple_span(strings[1].clone()),
                    Span::raw("!"),
                ])
            }
            _ => self.title = Line::from(DEFAULT_TITLE),
        }
    }

    pub fn search_input_history(&mut self, key_code: KeyCode) -> String {
        let pending: i32;
        if self.messages.lines.is_empty() || self.history.inputs.is_empty() {
            return String::new();
        }

        let first = self.history.check_first();

        match key_code {
            KeyCode::Right => pending = self.history.current + 1,
            KeyCode::Left => {
                if !first {
                    pending = self.history.current - 1;
                } else {
                    pending = self.history.current;
                }
            }
            _ => pending = 0,
        }

        let current = self.history.set_current_history_index(pending) as usize;
        self.history.inputs[current].clone()
    }
}
