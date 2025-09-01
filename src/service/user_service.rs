use crate::chat::chat_client::{self, ChatClient};
use crate::helper;
use crate::plugins::plugin_registry::{self, PluginRegistry};
use crate::types;
use crate::types::{Message, Response};
use std::fmt::format;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

#[derive(Debug)]
pub struct UserService {
    pub plugin_registry: Arc<PluginRegistry<'static>>,
    pub chat_client: Arc<ChatClient>,
}

impl UserService {
    pub fn new_user_service(chat_client: Arc<ChatClient>) -> UserService {
        UserService {
            plugin_registry: Arc::new(PluginRegistry::register_plugins(chat_client.clone())),
            chat_client,
        }
    }

    pub async fn executor(&self, input: &str) {
        let chat_client = self.chat_client.clone();
        let msg = chat_client.parse_input_to_message(input).await;
        // debug //
        let msg_clone = msg.clone();
        let _ = chat_client
            .output
            .send(Response {
                client_id: msg_clone.client_id,
                rsp_name: msg_clone.name,
                content: msg_clone.content,
                err: msg_clone.plugin,
            })
            .await;
        // debug //
        let mut err = String::new();

        let comment = match self.plugin_registry.find_and_execute(msg).await {
            Ok(rsp) => rsp,
            Err(e) => {
                err = format!("{}: {}", e.kind, e.msg);
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
