use crate::chat::chat_client::ChatClient;
use crate::helper;
use crate::types;
use crate::types::{Message, Response};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

// HttpClient handles all network related tasks
pub struct HttpClient {
    url: String,
    http_client: reqwest::Client,
    pub endpoints: HashMap<types::Route, String>,
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
            types::Route::PostRegister,
            format!("{}/users/{}", &url, client_id),
        );
        self.endpoints.insert(
            types::Route::PostPlugin,
            format!("{}/users/{}/run", &url, client_id),
        );
        self.endpoints.insert(
            types::Route::Delete,
            format!("{}/users/{}", &url, client_id),
        );
        self.endpoints.insert(
            types::Route::Get,
            format!("{}/users/{}/chat", &url, client_id),
        );
        self.endpoints.insert(
            types::Route::SignalWebRTC,
            format!("{}/users/{}/signal", &url, client_id),
        );
    }

    // GetRequest sends a GET Request to the server including the authorization token
    pub async fn get_request(&self, url: &String) -> Result<types::Response, reqwest::Error> {
        // FIXME client registered testen

        let auth_token = self.auth_token.lock().await.clone();
        let echo_json: types::Response = self
            .http_client
            .get(url)
            .header("Authorization", auth_token)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }

    // DeleteRequest sends a DELETE Request to delete the client out of the server
    // including the authorization token
    pub async fn delete_request(
        &self,
        url: &String,
        body: String,
    ) -> Result<types::Response, reqwest::Error> {
        let auth_token: String = self.auth_token.lock().await.clone();
        let echo_json: types::Response = self
            .http_client
            .delete(url)
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
    pub async fn post_request(
        &self,
        url: &String,
        body: String,
    ) -> Result<types::Response, reqwest::Error> {
        let auth_token: String = self.auth_token.lock().await.clone();
        let echo_json: types::Response = self
            .http_client
            .post(url)
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
