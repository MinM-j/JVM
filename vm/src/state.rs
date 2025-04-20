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

pub static GLOBAL_BOOL: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

pub static MEMORY_SIZE: Lazy<Arc<Mutex<usize>>> = Lazy::new(|| Arc::new(Mutex::new(1024)));

pub static MEMORY_SNAP: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

pub static FILE_NAME: Lazy<Arc<Mutex<String>>> = Lazy::new(|| Arc::new(Mutex::new("dump.json".to_string())));

pub static VIS_BOOL: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
