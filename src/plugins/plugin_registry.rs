use crate::chat::chat_client::ChatClient;
use crate::plugins::plugins;
use crate::types::{ChatError, ChatErrorWithMsg, Message};
use async_trait::async_trait;
use core::fmt::Debug;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

#[async_trait]
pub trait PluginTrait: Send + Sync {
    // + Send + Sync um Errors Thread Safe zu machen
    async fn execute(&self, msg: Message) -> Result<String, ChatErrorWithMsg>;
}

impl Debug for dyn PluginTrait {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PluginTrait (async trait object)")
    }
}

#[derive(Debug)]
pub struct PluginRegistry<'a> {
    pub plugins: HashMap<&'a str, Box<dyn PluginTrait>>,
    chat_client: Arc<ChatClient>,
    pub forward_plugins: Vec<&'a str>,
}

impl<'a> PluginRegistry<'a> {
    pub fn register_plugins(chat_client: Arc<ChatClient>) -> PluginRegistry<'static> {
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

    pub async fn find_and_execute(&self, msg: Message) -> Result<String, ChatErrorWithMsg> {
        let command = msg.plugin.as_str();
        if command != "/register" && !*self.chat_client.registered.lock().await {
            return Err(ChatErrorWithMsg::new(
                ChatError::NoPermission,
                String::from("You have to be registered"),
            ));
        }

        if command == "/register" && *self.chat_client.registered.lock().await {
            return Err(ChatErrorWithMsg::new(
                ChatError::NoPermission,
                String::from("You are already registered"),
            ));
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
                Err(err) => Err(err),
            },
            None => Err(ChatErrorWithMsg::new(
                ChatError::EmptyField,
                format!("Plugin field in message is empty {:?}", msg),
            )),
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
