use crate::chat::chat_client::ChatClient;
use crate::helper;
use crate::types;
use crate::types::{HttpClientError, Message, Response};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

// HttpClient handles all network related tasks
pub struct HttpClient {
    url: String,
    http_client: reqwest::Client,
    pub endpoints: HashMap<types::Endpoint, String>,
    auth_token: Arc<Mutex<String>>,
}

impl HttpClient {
    pub fn new_client(
        server_url: String,
        auth_token: Arc<Mutex<String>>,
        client_id: Arc<Mutex<String>>,
    ) -> HttpClient {
        let mut client = HttpClient {
            url: server_url.clone(),
            http_client: reqwest::Client::new(),
            endpoints: HashMap::new(),
            auth_token,
        };

        client.register_endpoints(server_url, client_id);

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
    ) -> Result<types::Response, Box<dyn Error>> {
        // FIXME client registered testen in response poller

        let endpoint_url = match self.endpoints.get(&endpoint) {
            Some(e) => e,
            None => return Err(Box::new(HttpClientError::InvalidEndpoint)),
        };
        let auth_token = self.auth_token.lock().await.clone();

        let echo_json: types::Response = self
            .http_client
            .get(endpoint_url)
            .header("Authorization", auth_token)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }

    // DeleteRequest sends a DELETE Request to delete the client out of the server
    // including the authorization token
    pub async fn delete_request(&self, msg: Message) -> Result<types::Response, Box<dyn Error>> {
        let body = serde_json::to_string(&msg)?;
        let endpoint_url = match self.endpoints.get(&types::Endpoint::Delete) {
            Some(e) => e,
            None => return Err(Box::new(HttpClientError::InvalidEndpoint)),
        };

        let auth_token: String = self.auth_token.lock().await.clone();

        let echo_json: types::Response = self
            .http_client
            .delete(endpoint_url)
            .header("Authorization", auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }

    // PostReqeust sends a Post Request to send a message to the server
    // including the authorization token
    pub async fn post_message(
        &self,
        endpoint: types::Endpoint,
        msg: Message,
    ) -> Result<types::Response, Box<dyn Error>> {
        let body = serde_json::to_string(&msg)?;
        let endpoint_url = match self.endpoints.get(&endpoint) {
            Some(e) => e,
            None => return Err(Box::new(HttpClientError::InvalidEndpoint)),
        };
        let auth_token: String = self.auth_token.lock().await.clone();

        let echo_json: types::Response = self
            .http_client
            .post(endpoint_url)
            .header("Authorization", auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }
}
