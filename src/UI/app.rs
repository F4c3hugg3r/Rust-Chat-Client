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
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    pub input: Vec<String>,
    pub messages: Vec<String>,
    pub user_service: UserService,
    pub receiver: Receiver<Response>,
    pub text_input: TextArea<'a>,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub chat_size: Size,
    pub title: String,
}

// TODO weitere App Logik hinzufügen
// TODO Title
// TODO Message Polling
// TODO input history
// TODO tabelle?
// TODO webrtc?
impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(user_service: UserService, receiver: Receiver<Response>) -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            input: Vec::new(),
            messages: Vec::new(),
            user_service,
            receiver,
            text_input: TextArea::default(),
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            chat_size: Size::new(0, 0),
            title: String::from(
                "Insert Title here.\nPress `Esc`, `Ctrl-C` or `q` to stop running.\n",
            ),
        }
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

    pub async fn handle_message(&mut self) {
        //save Input
        let input_text = self.text_input.lines().join("\n");
        self.messages.push(input_text);
        // TODO fix scrolling
        if self.messages.len()
            >= (self.chat_size.height as usize).saturating_sub(self.title.lines().count())
        {
            self.vertical_scroll = self.messages.len();
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        }
        self.text_input = TextArea::default(); // oder .clear()
        // self.user_service.executor(input_text.as_str()).await;
    }
}
