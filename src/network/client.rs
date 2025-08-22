use crate::helper;
use crate::network::http;
use crate::types;
use crate::types::{Message, Response};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{Sender, channel};
use tokio::sync::{Mutex, Notify};

// Client handles all network tasks
pub struct Client {
    pub client_name: Arc<Mutex<String>>,
    pub client_id: Arc<Mutex<String>>,
    pub auth_token: Arc<Mutex<String>>,
    // group_id: String,
    pub registered: Arc<Mutex<bool>>,
    // pub current_calling: String,
    notify: Notify,
    output: Sender<Response>,

    // TODO channel für output und tx (sender) könne mehrere sein / rx (receiver) (gibt nur einen receiver)
    //  channel wird außerhalb initialisiert: überlegen wie am besten
    // pub output: mpsc::Sender<types::Response>

    // LogChan                chan t.Log
    // ClientChangeSignalChan chan t.ClientsChangeSignal
    // CallTimeoutChan        chan bool
    pub url: String,
    pub http_client: reqwest::Client,
    pub endpoints: HashMap<types::Route, String>,
    // PortAudioMicInput *a.PortAudioMicInput,
    // SpeakerOutput     *a.SpeakerOutput,

    // Peers map[string]*Peer,
}

impl Client {
    pub fn new_client(server_url: String, tx: Sender<Response>) -> Client {
        let mut client = Client {
            // TODO auch client_id setzen bei Registrierung
            client_id: Arc::new(Mutex::new(String::from(""))),
            client_name: Arc::new(Mutex::new(String::from(""))),
            auth_token: Arc::new(Mutex::new(String::from(""))),
            registered: Arc::new(Mutex::new(false)),
            url: server_url.clone(),
            http_client: reqwest::Client::new(),
            endpoints: HashMap::new(),
            output: tx,
            notify: Notify::new(),
        };

        client.register_endpoints(server_url);

        client
    }

    // RegisterEndpoints registeres endpoint urls to the corresponding enum values
    async fn register_endpoints(&mut self, url: String) {
        let client_id = self.client_id.lock().await.clone();
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

    async fn register(&self, rsp: types::Response, new_id: String) {
        let mut client_name = self.client_name.lock().await;
        let mut auth_token = self.auth_token.lock().await;
        let mut client_id = self.client_id.lock().await;
        let mut registered = self.registered.lock().await;

        *client_name = rsp.rsp_name;
        *auth_token = rsp.content;
        *client_id = new_id;
        *registered = true;

        self.notify.notify_waiters();
    }

    async fn unregister(&self) {
        let mut client_name = self.client_name.lock().await;
        let mut auth_token = self.auth_token.lock().await;
        let mut client_id = self.client_id.lock().await;
        let mut registered = self.registered.lock().await;

        *client_name = String::from("");
        *auth_token = String::from("");
        *client_id = String::from("");
        *registered = false;
    }

    async fn check_registered(&self) {
        loop {
            {
                let registered = self.registered.lock().await;
                if *registered {
                    break;
                }
            }
            self.notify.notified().await;
        }
    }

    pub async fn response_receiver(&self, url: &String) {
        loop {
            self.check_registered().await;

            match self.get_request(url).await {
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

    // TODO Interrupt
}
