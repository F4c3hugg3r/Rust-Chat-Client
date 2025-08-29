use crate::service::user_service::UserService;
use crate::{
    UI::event::{AppEvent, Event, EventHandler},
    UI::user_interface,
    types::Response,
};
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
}

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
        }
    }

    // Event parsing
    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                user_interface::render_ui(&mut self, frame.area(), frame.buffer_mut())
            })?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => {
                        self.handle_key_events(key_event)?;
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
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
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
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

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub async fn handle_message(&mut self) {
        //save Input
        let input_text = self.text_input.lines().join("\n");
        self.messages.push(input_text);
        self.text_input = TextArea::default(); // oder .clear()
        // self.user_service.executor(input_text.as_str()).await;
    }
}
