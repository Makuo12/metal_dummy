use std::{
    mem, sync::{atomic::AtomicBool, Condvar, Mutex, RwLock}, time::Instant,
};

use heapless::Vec;

use crate::{constant::{MsgPipe, PricePipe}, lcd::task::LCDCommand};

#[derive(Debug, Clone)]
pub struct Conn {
    pub id: u16,
    pub data: MsgPipe,
    pub price: PricePipe,
    pub price_read: usize,
    pub time: Instant,
}

impl Conn {
    pub fn new(id: u16, price: PricePipe) -> Self {
        Conn {
            id,
            data: heapless::Vec::new(),
            price: price,
            price_read: 0,
            time: Instant::now(),
        }
    }
    pub fn take_data(&mut self) -> Vec<Vec<u8, 16>, 20> {
        mem::take(&mut self.data)
    }
}


pub struct MainState {
    pub can_type: Mutex<bool>,
    pub cond_var: Condvar,
    pub sent: AtomicBool,
    pub wifi_access: AtomicBool,
    pub msg: RwLock<heapless::Vec<char, 11>>,
    pub pipe: Mutex<Option<(MsgPipe, i64)>>,
    pub cond_var_pipe: Condvar,
    pub lcd_command: Mutex<Option<LCDCommand>>, 
    pub cond_var_lcd: Condvar,
}

impl MainState {
    pub fn new(wifi_access: AtomicBool) -> Self {
        Self {
            can_type: Mutex::new(true),
            cond_var: Condvar::new(),
            sent: AtomicBool::new(false),
            wifi_access: wifi_access,
            msg: RwLock::new(heapless::Vec::new()),
            pipe: Mutex::new(None),
            cond_var_pipe: Condvar::new(),
            lcd_command: Mutex::new(None),
            cond_var_lcd: Condvar::new(),
        }
    }
}
