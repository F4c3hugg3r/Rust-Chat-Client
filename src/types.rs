use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// routes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Endpoint {
    PostPlugin,
    PostRegister,
    Delete,
    Get,
    SignalWebRTC,
}

#[derive(Error, Debug)]
#[error("An ChatError accured")]
pub struct ChatErrorWithMsg {
    pub msg: String,
    pub kind: ChatError,
}

impl ChatErrorWithMsg {
    pub fn new(kind: ChatError, msg: String) -> Self {
        Self { kind, msg }
    }
}

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("A message field is empty")]
    EmptyField,
    #[error("You got no permission")]
    NoPermission,
    #[error("Item is not available")]
    NotAvailable,
    #[error("Timeout has been reached")]
    TimeoutReached,
    #[error("Your input was invalid")]
    WrongInput,
    #[error("An error accured while executing a plugin")]
    PluginError,
    #[error("An http error accured")]
    HttpError,
}

// #[derive(Error, Debug)]
// pub enum CustomError {
//     #[error("A message field is empty")]
//     EmptyMessageField,
// }

#[derive(Error, Debug)]
pub enum HttpClientError {
    #[error("This endpoint is invalid")]
    InvalidEndpoint,
}

pub fn dummy_json_client() -> JsonClient {
    JsonClient {
        name: "Bitte registriere dich, um die Clients zu sehen.".to_string(),
        call_state: "".to_string(),
        client_id: "".to_string(),
        group_name: "".to_string(),
        group_id: "".to_string(),
    }
}

// Farbkonstanten
pub const RED_COLOR: Color = Color::Rgb(191, 53, 53);
pub const BLUE_COLOR: Color = tailwind::BLUE.c400;
pub const PURPLE_COLOR: Color = tailwind::PURPLE.c950;
pub const TURKIS_COLOR: Color = Color::Rgb(53, 191, 188);
pub const DARK_TURKIS_COLOR: Color = tailwind::CYAN.c800;
pub const GREEN_COLOR: Color = Color::Rgb(62, 138, 41);
pub const DARK_YELLOW_COLOR: Color = tailwind::YELLOW.c950;

pub const DEFAULT_TITLE: &str = "Willkommen im Chatraum!";
pub const DEFAULT_MESSAGE: &str = "-> Schreibe '/register {name}' um dich zu registrieren";
pub const WINDOW_RESIZE_FLAG: &str = "windowResize";
pub const REGISTER_OUTPUT: &str = "-> Du kannst nun Nachrichten schreiben oder Commands ausführen";
pub const REGISTER_HELP_OUTPUT: &str = "[ '/help' → Befehle anzeigen ]";
pub const REGISTER_QUIT_OUTPUT: &str = "[ '/quit' → Chat verlassen ]";

// muteable device
pub const MICROPHONE: &str = "Mic";
pub const SPEAKER: &str = "Speaker";

pub const UNREGISTER_FLAG: &str = "- Du bist nun vom Server getrennt -";
pub const REGISTER_FLAG: &str = "- Du bist registriert -";
pub const ADD_GROUP_FLAG: &str = "Add Group";
pub const LEAVE_GROUP_FLAG: &str = "Leave Group";

pub const USERS_FLAG: &str = "Users";
pub const IGNORE_RESPONSE_TAG: &str = "Ignore Response";
pub const USER_ADD_FLAG: &str = "Add User";
pub const USER_REMOVE_FLAG: &str = "Remove User";

// signal flags
pub const ICE_CANDIDATE_FLAG: &str = "ICE Candidate";
pub const ROLLBACK_DONE_FLAG: &str = "Rollback Done";
pub const INITIALIZE_SIGNAL_FLAG: &str = "Initialize Call";
pub const CALL_ACCEPTED: &str = "Call Accepted";
pub const CALL_DENIED: &str = "Call denied";
pub const RECEIVE_CALL: &str = "ReceiveCall";

// signalflags -> callStates
pub const OFFER_SIGNAL_FLAG: &str = "Offer Signal";
pub const ANSWER_SIGNAL_FLAG: &str = "Answer Signal";
pub const STABLE_SIGNAL_FLAG: &str = "Stable Flag";
pub const CONNECTED_FLAG: &str = "Connected";
pub const FAILED_CONNECTION_FLAG: &str = "Connection Failed";
pub const NO_CALL_FLAG: &str = "No Call";

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
    // #[serde(rename = "groupId")]
    // pub group_id: String,
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

impl Response {
    pub fn empty() -> Self {
        Response {
            client_id: String::new(),
            rsp_name: String::from("empty Response"),
            content: String::new(),
            err: String::new(),
        }
    }

    pub fn error(error: String) -> Self {
        Response {
            client_id: String::new(),
            rsp_name: String::from("error Response"),
            content: String::new(),
            err: error,
        }
    }
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

// // signals, that a state of a client or the list of clients has changed
// #[derive(Debug, Clone)]
// pub struct ClientsChangeSignal {
//     pub clients_json: String,
//     pub call_state: String,
//     pub opp_id: String,
// }
