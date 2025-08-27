use std::sync::{Arc, RwLock};

use crate::state::Conn;

// pub const ID_KEY: &[u8] = b"x3KnSDgjmcKJ0ELD";

pub type ConnArc = Arc<RwLock<heapless::Vec<Conn, 40>>>;
pub type MsgPipe =heapless::Vec<heapless::Vec<u8, 16>, 20>; 
pub type PricePipe =heapless::Vec<heapless::Vec<u8, 16>, 5>; 
pub type StatusArc = Arc<RwLock<heapless::Vec<(u16, u8), 40>>>;


