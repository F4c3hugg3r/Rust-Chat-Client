use crate::helper;
use crate::network::http_client::HttpClient;
use crate::types;
use crate::types::{Message, Response};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

// Client handles all network tasks
pub struct ChatClient {
    pub client_name: Arc<Mutex<String>>,
    pub client_id: Arc<Mutex<String>>,
    pub auth_token: Arc<Mutex<String>>,
    // group_id: String,
    pub registered: Arc<Mutex<bool>>,
    pub output: Sender<Response>,
    pub http_client: HttpClient,
    // pub current_calling: String,
    notify: Notify,
    // TODO channel für output und tx (sender) könne mehrere sein / rx (receiver) (gibt nur einen receiver)
    //  channel wird außerhalb initialisiert: überlegen wie am besten
    // pub output: mpsc::Sender<types::Response>

    // LogChan                chan t.Log
    // ClientChangeSignalChan chan t.ClientsChangeSignal
    // CallTimeoutChan        chan bool
    // PortAudioMicInput *a.PortAudioMicInput,
    // SpeakerOutput     *a.SpeakerOutput,

    // Peers map[string]*Peer,
}

impl ChatClient {
    pub fn new_client(server_url: String, tx: Sender<Response>) -> ChatClient {
        let client_id = Arc::new(Mutex::new(String::from("")));
        let auth_token = Arc::new(Mutex::new(String::from("")));
        ChatClient {
            // TODO auch client_id setzen bei Registrierung
            client_id: client_id.clone(),
            client_name: Arc::new(Mutex::new(String::from(""))),
            auth_token: auth_token.clone(),
            registered: Arc::new(Mutex::new(false)),
            output: tx,
            notify: Notify::new(),
            http_client: HttpClient::new_client(server_url, auth_token, client_id),
        }
    }

    pub async fn register(&self, rsp: types::Response, new_id: String) {
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

    pub async fn unregister(&self) {
        let mut client_name = self.client_name.lock().await;
        let mut auth_token = self.auth_token.lock().await;
        let mut client_id = self.client_id.lock().await;
        let mut registered = self.registered.lock().await;

        *client_name = String::from("");
        *auth_token = String::from("");
        *client_id = String::from("");
        *registered = false;
    }

    pub async fn check_registered(&self) {
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
}
