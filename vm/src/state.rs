use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

#[derive(Clone, Debug)]
pub enum Header {
    DATA,
    EOF,
}

#[derive(Debug, Clone)]
pub struct MessageData {
    pub header: Header,
    pub json: String,
}

pub static SERVER_STATE: Lazy<Arc<Mutex<VecDeque<MessageData>>>> =
    Lazy::new(|| Arc::new(Mutex::new(VecDeque::new())));
