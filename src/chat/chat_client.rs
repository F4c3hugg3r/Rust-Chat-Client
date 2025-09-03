use crate::helper;
use crate::network::http_client::HttpClient;
use crate::types::{self, ChatErrorWithMsg, JsonGroup};
use crate::types::{Message, Response};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

#[derive(Debug)]
// Client handles all network tasks
pub struct ChatClient {
    pub client_name: Arc<Mutex<String>>,
    pub client_id: Arc<Mutex<String>>,
    pub auth_token: Arc<Mutex<String>>,
    pub group: Arc<Mutex<Option<JsonGroup>>>,
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
    pub async fn new_client(server_url: String, tx: Sender<Response>) -> Self {
        let client_id = Arc::new(Mutex::new(helper::generate_secure_token(32)));
        let auth_token = Arc::new(Mutex::new(String::from("")));
        Self {
            client_id: client_id.clone(),
            client_name: Arc::new(Mutex::new(String::new())),
            group: Arc::new(Mutex::new(None)),
            auth_token: auth_token.clone(),
            registered: Arc::new(Mutex::new(false)),
            output: tx,
            notify: Notify::new(),
            http_client: HttpClient::new_client(server_url, auth_token, client_id).await,
        }
    }

    pub async fn register(&self, rsp: types::Response) {
        let mut client_name = self.client_name.lock().await;
        let mut auth_token = self.auth_token.lock().await;
        let mut registered = self.registered.lock().await;

        *client_name = rsp.rsp_name;
        *auth_token = rsp.content;
        *registered = true;

        self.notify.notify_waiters();
    }

    pub async fn unregister(&self) {
        let mut client_name = self.client_name.lock().await;
        let mut auth_token = self.auth_token.lock().await;
        let mut registered = self.registered.lock().await;
        let mut group_id = self.group.lock().await;

        *group_id = None;
        *client_name = String::from("");
        *auth_token = String::from("");
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

    pub async fn handle_add_group(
        &self,
        group_json: String,
    ) -> Result<JsonGroup, ChatErrorWithMsg> {
        let group: JsonGroup = match serde_json::from_str(group_json.as_str()) {
            Ok(g) => g,
            Err(e) => {
                return Err(ChatErrorWithMsg::new(
                    types::ChatError::WrongInput,
                    e.to_string(),
                ));
            }
        };

        let _ = self.group.lock().await.insert(group.clone());

        Ok(group)
    }
}
