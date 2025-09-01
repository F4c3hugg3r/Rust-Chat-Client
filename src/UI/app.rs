use std::sync::Arc;

use crate::service::user_service::UserService;
use crate::{
    UI::event::{AppEvent, Event, EventHandler},
    UI::user_interface,
    types::Response,
};
use ratatui::layout::Size;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use tokio::sync::mpsc::Receiver;
use tui_textarea::TextArea;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    pub messages: Vec<String>,
    pub user_service: Arc<UserService>,
    pub text_input: TextArea<'static>,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub chat_size: Size,
    pub title: String,
}

// TODO Title
// TODO input history
// TODO tabelle?
// TODO webrtc?
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(user_service: Arc<UserService>, mut receiver: Receiver<Response>) -> Self {
        let mut app = Self {
            running: true,
            events: EventHandler::new(),
            messages: Vec::new(),
            user_service: user_service.clone(),
            text_input: TextArea::default(),
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            chat_size: Size::new(0, 0),
            title: String::from(
                "Insert Title here.\nPress `Esc`, `Ctrl-C` or `q` to stop running.\n",
            ),
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
        self.messages.push(format!(
            "name: {} | content: {} | error/plugin: {} | id: {}",
            rsp.rsp_name, rsp.content, rsp.err, rsp.client_id,
        ));
        self.scroll();
    }

    pub async fn handle_message(&mut self) {
        self.user_service
            .executor(self.text_input.lines().join("\n").as_str())
            .await;
        self.text_input = TextArea::default(); // oder .clear()
    }

    pub fn scroll(&mut self) {
        let title_rows = self.title.lines().count().saturating_add(4);
        let message_rows = self.chat_size.height.saturating_sub(title_rows as u16) as usize;

        if self.messages.len() > message_rows {
            self.vertical_scroll = self.messages.len() - message_rows;
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        }
    }
}
