use std::collections::HashMap;

// Client handles all network tasks
pub struct Client<'a> {
    client_name: String,
    client_id: String,
    pub auth_token: String,
    group_id: String,
    pub registered: bool,
    // pub current_calling: String,

    // mu                     *sync.RWMutex
    // cond                   *sync.Cond
    // Output                 chan *t.Response
    // LogChan                chan t.Log
    // ClientChangeSignalChan chan t.ClientsChangeSignal
    // CallTimeoutChan        chan bool
    pub url: String,
    pub http_client: &'a reqwest::Client,
    pub endpoints: HashMap<i32, String>,
    // PortAudioMicInput *a.PortAudioMicInput,
    // SpeakerOutput     *a.SpeakerOutput,

    // Peers map[string]*Peer,
}
