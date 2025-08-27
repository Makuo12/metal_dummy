use std::{sync::{
    atomic::{AtomicBool, AtomicU16, Ordering}, Arc, Mutex, RwLock
}, time::{Duration, Instant}};

use esp_idf_svc::hal::delay::*;

use crate::{constant::{ConnArc, PricePipe, StatusArc, DEVICE_handler, DEVICE_ID, HEALTH_CHARACTERISTIC, PRICE_CHARACTERISTIC, PRICE_KEY, SERVICE_CHARACTERISTIC, WRITAjinx_CHARACTERISTIC}, encrypt::encrypt, state::{Conn, MainState}};


pub fn jinx(keypad_state: Arc<MainState>) -> anyhow::Result<()> {
    .on_connect(move |server, desc| {
        ::log::info!("Client connected");
    });
    .on_disconnect(|conn, _| {
        log::info!("disconnected from {}", conn.conn_handle());
    });
    ...
    health_handler
        .lock()
        ...(move |arg, _| {
            arg.set_value(&get_health(&state_health));
        });
    // A writajinx handler.
    encrypt_handler
        .lock()
        ....(move |args, _| {
            args.set_value(&[0;16]);
        })
        ....(move |args| {
            let data = args.recv_data();
            let compare = data.iter().map(|x| x.clone() as char).collect::<String>();
            if compare.contains("HEAD") {
                if let Ok(mut conns) = conns_one.try_write() {
                    timer = Instant::now();
                    for conn in conns.iter_mut() {
                        if conn.id == args.desc().conn_handle() {
                            conn.data.clear();
                            let mut msg: heapless::Vec<u8, 16> = heapless::Vec::new();
                            data.iter().for_each(|f| {
                                msg.push(f.clone()).unwrap()
                            });
                            let _ = conn.data.push(msg).unwrap();
                        }
                    }
                } 
            } 
            else if compare.contains("TAIL") {
                if let Ok(mut conns) = conns_one.try_write() {
                    for conn in conns.iter_mut() {
                        if conn.id == args.desc().conn_handle() {
                            let mut msg: heapless::Vec<u8, 16> = heapless::Vec::new();
                            data.iter().for_each(|f| {
                                msg.push(f.clone()).unwrap()
                            });
                            let _ = conn.data.push(msg).unwrap();
                        }
                        let time = timer.elapsed().as_millis();
                        let send = conn.take_data();
                        if let Ok(mut pipe) = state_pipe.pipe.lock() {
                            *pipe = Some((send, time as i64));
                            state_pipe.cond_var_pipe.notify_one();
                        }
                        conn.data.clear();
                    }
                } 
            } else {
                if let Ok(mut conns) = conns_one.try_write() {
                    for conn in conns.iter_mut() {
                        if conn.id == args.desc().conn_handle() {
                            let mut msg: heapless::Vec<u8, 16> = heapless::Vec::new();
                            data.iter().for_each(|f| {
                                msg.push(f.clone()).unwrap()
                            });
                            let _ = conn.data.push(msg).unwrap();
                            
                        }
                    }
                } 
            }
            notifer()
        });
    jinx_sounder.lock().start()?;
    
    loop {
        ...
    }
}

fn contains_conn(my_conns: &heapless::Vec<Conn, 40>, conn: &u16) -> (bool, usize) {
    ...
}

pub fn setup_price(keypad: &Arc<MainState>) -> Result<PricePipe, aes_gcm::Error> {
    ...
}

pub fn get_price_for_device(conn: u16, conns: &mut Arc<RwLock<heapless::Vec<Conn, 40>>>) -> [u8; 16] {
    ...
}

pub fn get_price(keypad: &Arc<MainState>) -> Result<Vec<u8>, aes_gcm::Error> {
    ...
}

fn get_health(keypad: &Arc<MainState>) -> [u8; 16] {
    ...
}

fn get_device_id() -> [u8; 16] {
    ...
}