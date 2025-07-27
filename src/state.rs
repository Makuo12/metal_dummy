use std::{
    sync::{atomic::AtomicBool, Condvar, Mutex, RwLock},
    time::Instant,
};

#[derive(Debug, Copy, Clone)]
pub struct Conn {
    pub id: u16,
    pub time: Instant,
}

impl Conn {
    pub fn new(id: u16) -> Self {
        Conn {
            id,
            time: Instant::now(),
        }
    }
}

pub struct KeypadState {
    pub can_type: Mutex<bool>,
    pub cond_var: Condvar,
    pub has_msg: RwLock<bool>,
    pub cancel: AtomicBool,
    pub sent: AtomicBool,
    pub msg: RwLock<heapless::Vec<char, 11>>,
}

impl KeypadState {
    pub fn new() -> Self {
        Self {
            can_type: Mutex::new(true),
            cond_var: Condvar::new(),
            has_msg: RwLock::new(false),
            cancel: AtomicBool::new(false),
            sent: AtomicBool::new(false),
            msg: RwLock::new(heapless::Vec::new()),
        }
    }
}
