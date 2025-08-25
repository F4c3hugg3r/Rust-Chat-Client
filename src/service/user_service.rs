use crate::chat::chat_client::ChatClient;
use crate::helper;
use crate::plugins::plugin_registry::PluginRegistry;
use crate::types;
use crate::types::{Message, Response};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

struct user_servcice {
    plugin_registry: PluginRegistry<'static>,
    chat_client: ChatClient,
}

// TODO Executor

// TODO Interrupt
