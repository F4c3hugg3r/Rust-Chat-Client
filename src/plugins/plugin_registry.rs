use crate::chat::chat_client::ChatClient;
use crate::plugins::plugins;
use crate::types::{ChatError, Message};
use async_trait::async_trait;
use core::fmt::Debug;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

#[async_trait]
pub trait PluginTrait {
    async fn execute(&self, msg: Message) -> Result<String, Box<dyn Error>>;
}

impl Debug for dyn PluginTrait {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PluginTrait (async trait object)")
    }
}

#[derive(Debug)]
pub struct PluginRegistry<'a> {
    pub plugins: HashMap<&'a str, Box<dyn PluginTrait>>,
    chat_client: Arc<Mutex<ChatClient>>,
    pub forward_plugins: Vec<&'a str>,
}

impl<'a> PluginRegistry<'a> {
    pub fn register_plugins(chat_client: Arc<Mutex<ChatClient>>) -> PluginRegistry<'static> {
        let mut pr = PluginRegistry {
            chat_client,
            plugins: HashMap::new(),
            forward_plugins: Vec::new(),
        };
        pr.plugins.insert(
            "/register",
            Box::new(plugins::RegisterClientPlugin::new_register_client_plugin(
                pr.chat_client.clone(),
            )),
        );
        pr.plugins.insert(
            "/quit",
            Box::new(plugins::LogOutPlugin::new_logout_plugin(
                pr.chat_client.clone(),
            )),
        );
        pr.plugins.insert(
            "/private",
            Box::new(plugins::PrivateMessagePlugin::new_private_message_plugin(
                pr.chat_client.clone(),
            )),
        );
        pr.plugins.insert(
            "",
            Box::new(plugins::ForwardPlugin::new_forward_plugin(
                pr.chat_client.clone(),
            )),
        );
        // pr.plugins.insert(
        //     "/call",
        //     Box::new(plugins::RegisterClientPlugin::new_register_client_plugin(
        //         pr.chat_client.clone(),
        //     )),
        // );
        pr.fill_forward_plugins();
        pr
    }

    pub async fn find_and_execute(&self, msg: Message) -> Result<String, Box<dyn Error>> {
        let command = msg.plugin.as_str();
        if command != "/register" && !*self.chat_client.lock().await.registered.lock().await {
            return Err(Box::new(ChatError::NoPermission {
                msg: String::from("You have to be registered"),
            }));
        }

        if command == "/register" && *self.chat_client.lock().await.registered.lock().await {
            return Err(Box::new(ChatError::NoPermission {
                msg: String::from("You are already registered"),
            }));
        }

        if command.is_empty() {
            return Ok(String::from(""));
        }

        let new_command = if self.forward_plugins.contains(&command) {
            ""
        } else {
            command
        };

        let plugin = self.plugins.get(new_command);
        match plugin {
            Some(plugin) => match plugin.execute(msg).await {
                Ok(str) => Ok(str),
                Err(e) => Err(e),
            },
            None => Err(Box::new(ChatError::EmptyField {
                msg: format!("Plugin field in message is empty {:?}", msg),
            })),
        }
    }

    pub fn fill_forward_plugins(&mut self) {
        self.forward_plugins.push("/help");
        self.forward_plugins.push("/time");
        self.forward_plugins.push("/users");
        self.forward_plugins.push("/broadcast");
        self.forward_plugins.push("/group");
    }
}
