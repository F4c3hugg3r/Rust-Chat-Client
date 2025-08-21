// reqwest for asynchronous http requests
// serde for json serializing
// tokio for conurrency

// Beipsiel Post Req
// #[derive(Serialize)]
// struct MyStruct {
//     field: String,
// }

// let client = reqwest::Client::new();
// let res = client.post("https://example.com")
//     .json(&my_struct) // encodiert als JSON
//     .send()
//     .await?;

// Beispiel Get Req
// #[derive(Deserialize)]
// struct MyStruct {
//     field: String,
// }

// let res = client.get("https://example.com")
//     .send()
//     .await?
//     .json::<MyStruct>() // decodiert JSON zu Struct
//     .await?;

use serde::{Deserialize, Serialize};

// routes
enum Route {
    PostPlugin,
    PostRegister,
    Delete,
    Get,
    SignalWebRTC,
}

// muteable device
const MICROPHONE: &str = "Mic";
const SPEAKER: &str = "Speaker";

const UNREGISTER_FLAG: &str = "- Du bist nun vom Server getrennt -";
const REGISTER_FLAG: &str = "- Du bist registriert -";
const ADD_GROUP_FLAG: &str = "Add Group";
const LEAVE_GROUP_FLAG: &str = "Leave Group";

const USERS_FLAG: &str = "Users";
const IGNORE_RESPONSE_TAG: &str = "Ignore Response";
const USER_ADD_FLAG: &str = "Add User";
const USER_REMOVE_FLAG: &str = "Remove User";

// signal flags
const ICE_CANDIDATE_FLAG: &str = "ICE Candidate";
const ROLLBACK_DONE_FLAG: &str = "Rollback Done";
const INITIALIZE_SIGNAL_FLAG: &str = "Initialize Call";
const CALL_ACCEPTED: &str = "Call Accepted";
const CALL_DENIED: &str = "Call denied";
const RECEIVE_CALL: &str = "ReceiveCall";

// signalflags -> callStates
const OFFER_SIGNAL_FLAG: &str = "Offer Signal";
const ANSWER_SIGNAL_FLAG: &str = "Answer Signal";
const STABLE_SIGNAL_FLAG: &str = "Stable Flag";
const CONNECTED_FLAG: &str = "Connected";
const FAILED_CONNECTION_FLAG: &str = "Connection Failed";
const NO_CALL_FLAG: &str = "No Call";

// Message contains the name and id of the requester and the message (content) itsself
// as well as the uses plugin and groupId if a user is in a group
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "plugin")]
    pub plugin: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

// Response contains the name and id of the sender, the response (content) itself
// and an error string
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "name")]
    pub rsp_name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "errorString")]
    pub err: String,
}

// JsonGroup contains an id, the groupname and the size of the group
// Notice that there is another Group struct in groupRegistry which
// has some extra fields which are used for server internal logic
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonGroup {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonClient {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "callState")]
    pub call_state: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

// signals, that a state of a client or the list of clients has changed
#[derive(Debug, Clone)]
pub struct ClientsChangeSignal {
    pub clients_json: String,
    pub call_state: String,
    pub opp_id: String,
}
