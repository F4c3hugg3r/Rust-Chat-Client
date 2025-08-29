mod UI;
mod chat;
mod helper;
mod network;
mod plugins;
mod service;
mod types;

use crate::{
    UI::app::App,
    chat::chat_client::{self, ChatClient},
    service::user_service::{self, UserService},
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let (tx, rx) = tokio::sync::mpsc::channel(1000);
    let server_url = String::from("http://localhost:8080");
    let chat_client = Arc::new(Mutex::new(ChatClient::new_client(server_url, tx)));
    let user_service = UserService::new_user_service(chat_client);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(user_service, rx).run(terminal).await;
    ratatui::restore();
    result
}
