use crate::chat::chat_client::{self, ChatClient};
use crate::helper;
use crate::plugins::plugin_registry::{self, PluginRegistry};
use crate::types;
use crate::types::{Message, Response};
use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

#[derive(Debug)]
pub struct UserService {
    plugin_registry: PluginRegistry<'static>,
    chat_client: Arc<Mutex<ChatClient>>,
}

impl UserService {
    pub fn new_user_service(chat_client: Arc<Mutex<ChatClient>>) -> UserService {
        UserService {
            plugin_registry: PluginRegistry::register_plugins(chat_client.clone()),
            chat_client,
        }
    }

    pub async fn executor(&self, input: &str) {
        let chat_client = self.chat_client.lock().await;
        let msg = chat_client.parse_input_to_message(input).await;
        let mut err = String::new();

        let comment = match self.plugin_registry.find_and_execute(msg).await {
            Ok(rsp) => rsp,
            Err(e) => {
                err = e.to_string();
                String::new()
            }
        };

        // TODO implement Logoutput channel to handle sending error
        let _ = chat_client
            .output
            .send(Response {
                client_id: String::new(),
                rsp_name: String::new(),
                content: comment,
                err,
            })
            .await;
    }
}

// TODO Interrupt
