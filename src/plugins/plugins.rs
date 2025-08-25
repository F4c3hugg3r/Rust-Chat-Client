use crate::helper::generate_secure_token;
use crate::types::{self, ChatError, Endpoint, Response};
use crate::{chat::chat_client::ChatClient, plugins::plugin_registry::PluginTrait, types::Message};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

pub struct PrivateMessagePlugin {
    chat_client: Arc<Mutex<ChatClient>>,
}

impl PrivateMessagePlugin {
    pub fn new_private_message_plugin(chat_client: Arc<Mutex<ChatClient>>) -> PrivateMessagePlugin {
        PrivateMessagePlugin { chat_client }
    }
}

#[async_trait]
impl PluginTrait for PrivateMessagePlugin {
    async fn execute(&self, msg: Message) -> Result<String, Box<dyn Error>> {
        return Err(Box::new(ChatError::WrongInput {
            msg: String::from("TODO"),
        }));
    }
}

pub struct LogOutPlugin {
    chat_client: Arc<Mutex<ChatClient>>,
}

impl LogOutPlugin {
    pub fn new_logout_plugin(chat_client: Arc<Mutex<ChatClient>>) -> LogOutPlugin {
        LogOutPlugin { chat_client }
    }
}

#[async_trait]
impl PluginTrait for LogOutPlugin {
    async fn execute(&self, msg: Message) -> Result<String, Box<dyn Error>> {
        return Err(Box::new(ChatError::WrongInput {
            msg: String::from("TODO"),
        }));
    }
}

pub struct RegisterClientPlugin {
    chat_client: Arc<Mutex<ChatClient>>,
}

impl RegisterClientPlugin {
    pub fn new_register_client_plugin(chat_client: Arc<Mutex<ChatClient>>) -> RegisterClientPlugin {
        RegisterClientPlugin { chat_client }
    }
}

#[async_trait]
impl PluginTrait for RegisterClientPlugin {
    async fn execute(&self, msg: Message) -> Result<String, Box<dyn Error>> {
        if msg.content.len() > 50 || msg.content.len() < 3 {
            return Err(Box::new(ChatError::WrongInput {
                msg: String::from("Your name has to be between 3 and 50 characters long"),
            }));
        }

        let chat_client = self.chat_client.lock().await;
        let rsp = chat_client
            .http_client
            .post_message(Endpoint::PostPlugin, msg)
            .await?;

        chat_client.register(rsp, generate_secure_token(32));

        Ok(types::REGISTER_FLAG.to_string())
    }
}

pub struct ForwardPlugin {
    chat_client: Arc<Mutex<ChatClient>>,
}

impl ForwardPlugin {
    pub fn new_forward_plugin(chat_client: Arc<Mutex<ChatClient>>) -> ForwardPlugin {
        ForwardPlugin { chat_client }
    }
}

#[async_trait]
impl PluginTrait for ForwardPlugin {
    async fn execute(&self, msg: Message) -> Result<String, Box<dyn Error>> {
        let rsp: Response = self
            .chat_client
            .lock()
            .await
            .http_client
            .post_message(Endpoint::PostPlugin, msg)
            .await?;
        return Ok(rsp.err);
    }
}
