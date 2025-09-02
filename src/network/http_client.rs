use crate::chat::chat_client::ChatClient;
use crate::helper::generate_secure_token;
use crate::types;
use crate::types::ChatError;
use crate::types::ChatErrorWithMsg;
use crate::types::{HttpClientError, Message, Response};
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

#[derive(Debug)]
// HttpClient handles all network related tasks
pub struct HttpClient {
    http_client: reqwest::Client,
    pub endpoints: HashMap<types::Endpoint, String>,
    auth_token: Arc<Mutex<String>>,
}

impl HttpClient {
    pub async fn new_client(
        server_url: String,
        auth_token: Arc<Mutex<String>>,
        client_id: Arc<Mutex<String>>,
    ) -> Self {
        let mut client = Self {
            http_client: reqwest::Client::new(),
            endpoints: HashMap::new(),
            auth_token,
        };

        client.register_endpoints(server_url, client_id).await;

        client
    }

    // RegisterEndpoints registeres endpoint urls to the corresponding enum values
    async fn register_endpoints(&mut self, url: String, client_id: Arc<Mutex<String>>) {
        let client_id = client_id.lock().await.clone();
        self.endpoints.insert(
            types::Endpoint::PostRegister,
            format!("{}/users/{}", &url, client_id),
        );
        self.endpoints.insert(
            types::Endpoint::PostPlugin,
            format!("{}/users/{}/run", &url, client_id),
        );
        self.endpoints.insert(
            types::Endpoint::Delete,
            format!("{}/users/{}", &url, client_id),
        );
        self.endpoints.insert(
            types::Endpoint::Get,
            format!("{}/users/{}/chat", &url, client_id),
        );
        self.endpoints.insert(
            types::Endpoint::SignalWebRTC,
            format!("{}/users/{}/signal", &url, client_id),
        );
    }

    // GetRequest sends a GET Request to the server including the authorization token
    pub async fn get_response(
        &self,
        endpoint: types::Endpoint,
    ) -> Result<types::Response, ChatErrorWithMsg> {
        // FIXME client registered testen in response poller

        let endpoint_url = match self.endpoints.get(&endpoint) {
            Some(e) => e,
            None => {
                return Err(ChatErrorWithMsg::new(
                    ChatError::HttpError,
                    "Invalid endpoint".to_string(),
                ));
            }
        };
        let auth_token = self.auth_token.lock().await.clone();

        let http_response = self
            .http_client
            .get(endpoint_url)
            .header("Authorization", auth_token)
            .send()
            .await
            .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string()))?;

        match http_response.error_for_status() {
            Ok(rsp) => rsp
                .json()
                .await
                .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string())),
            Err(e) => Err(ChatErrorWithMsg::new(ChatError::HttpError, e.to_string())),
        }
    }

    // DeleteRequest sends a DELETE Request to delete the client out of the server
    // including the authorization token
    pub async fn delete_request(&self, msg: Message) -> Result<types::Response, ChatErrorWithMsg> {
        let body = serde_json::to_string(&msg)
            .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string()))?;
        let endpoint_url = match self.endpoints.get(&types::Endpoint::Delete) {
            Some(e) => e,
            None => {
                return Err(ChatErrorWithMsg::new(
                    ChatError::HttpError,
                    "Invalid endpoint".to_string(),
                ));
            }
        };

        let auth_token: String = self.auth_token.lock().await.clone();

        let http_response = self
            .http_client
            .delete(endpoint_url)
            .header("Authorization", auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string()))?;

        match http_response.error_for_status() {
            Ok(rsp) => rsp
                .json()
                .await
                .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string())),
            Err(e) => Err(ChatErrorWithMsg::new(ChatError::HttpError, e.to_string())),
        }
    }

    // PostReqeust sends a Post Request to send a message to the server
    // including the authorization token
    pub async fn post_message(
        &self,
        endpoint: types::Endpoint,
        msg: Message,
    ) -> Result<types::Response, ChatErrorWithMsg> {
        let body = serde_json::to_string(&msg)
            .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string()))?;
        let endpoint_url = match self.endpoints.get(&endpoint) {
            Some(e) => e,
            None => {
                return Err(ChatErrorWithMsg::new(
                    ChatError::HttpError,
                    "Invalid endpoint".to_string(),
                ));
            }
        };
        let auth_token: String = self.auth_token.lock().await.clone();

        let http_response = self
            .http_client
            .post(endpoint_url)
            .header("Authorization", auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string()))?;

        match http_response.error_for_status() {
            Ok(rsp) => rsp
                .json()
                .await
                .map_err(|e| ChatErrorWithMsg::new(ChatError::HttpError, e.to_string())),
            Err(e) => Err(ChatErrorWithMsg::new(ChatError::HttpError, e.to_string())),
        }
    }
}
