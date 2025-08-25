use crate::helper;
use crate::network::http_client::HttpClient;
use crate::types;
use crate::types::{Endpoint, Message, Response};
use crate::{chat::chat_client::ChatClient, network::http_client};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::{Receiver, channel};
use tokio::sync::{Mutex, Notify};

impl ChatClient {
    pub async fn create_message(
        &self,
        name: String,
        plugin: String,
        content: String,
        client_id: String,
    ) -> Message {
        let client_name = self.client_name.lock().await.clone();
        let client_client_id = self.client_id.lock().await.clone();

        let msg_name = if client_name.is_empty() || *self.registered.lock().await {
            client_name
        } else {
            name
        };

        let msg_client_id = if client_id.is_empty() {
            client_client_id
        } else {
            client_id
        };

        Message {
            name: msg_name,
            content,
            plugin,
            client_id: msg_client_id,
        }
    }

    pub async fn response_receiver(&self, url: &String) {
        loop {
            self.check_registered().await;

            match self.http_client.get_response(Endpoint::Get).await {
                Ok(rsp) => {
                    let _ = self.output.send(rsp).await;
                }
                Err(_) => {
                    continue;
                    // TODO log channel
                    // let _ = self.debug.send(err.to_string() oder so).await;
                }
            }
        }
    }

    pub async fn parse_input_to_message(&self, input: &str) -> Message {
        let new_input = input.trim_end_matches("\n");

        let plugin = if new_input.starts_with("/") {
            new_input.split_whitespace().next().unwrap_or("")
        } else {
            "/broadcast"
        };

        let replaced = new_input.replace(plugin, "");
        let content = replaced.trim_start_matches(" ");

        self.create_message(
            String::from(""),
            plugin.to_string(),
            content.to_string(),
            String::from(""),
        )
        .await
    }
}
