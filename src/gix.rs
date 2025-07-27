use std::{
    sync::{
        atomic::{AtomicBool, AtomicU16, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};


use crate::{
    constant::{
        GIX_SERVICE, CANCEL_CODE, DEVICE_ID, ENCRYPTION_DEVICE_ID_KEY, ENCRYPTION_KEY_PRICE,
    },
    encrypt::{aes_cipher, confirm, DeviceUser},
    lcd::control::lcd_connect,
    state::{Conn, KeypadState},
};

type ConnArc = Arc<Mutex<heapless::Vec<Conn, 40>>>;

pub fn gix_dev(keypad_state: Arc<KeypadState>) -> anyhow::Result<()> {
    let gix_device = GIXDevice::take();
    let gix_advertising = gix_device.get_advertising();
    let server = gix_device.get_server();
    let connected_device_id = Arc::new(AtomicBool::new(false));
    let connected_price = Arc::new(AtomicBool::new(false));
    let conn_handle_device_id = Arc::new(AtomicU16::new(0));
    let conn_handle_price = Arc::new(AtomicU16::new(0));

    let connected_device_id_clone = connected_device_id.clone();
    let connected_price_clone = connected_price.clone();

    let conn_handle_device_id_clone = conn_handle_device_id.clone();
    let conn_handle_price_clone = conn_handle_price.clone();

    let driver = Arc::new(AtomicU16::new(0));
    let driver_connected = Arc::new(AtomicBool::new(false));

    let driver_one = driver.clone();
    let driver_two = driver.clone();
    let driver_three = driver.clone();

    let driver_connected_one = driver_connected.clone();
    let driver_connected_two = driver_connected.clone();
    // let driver_connected_three = driver_connected.clone();

    let keypad = keypad_state.clone();
    let keypad_two = keypad_state.clone();
    let conns_arc: ConnArc = Arc::new(Mutex::new(heapless::Vec::new()));
    let conn_arc_one = conns_arc.clone();
    // let conn_arc_two = conns_arc.clone();
    gix_device
        .set_power(PowerType::Default, PowerLevel::N0)
        .unwrap();
    server.on_connect(move |server, desc| {
        ::log::info!("Client connected");
    });
    server.on_disconnect(move |conn, _| {
        if driver_connected_two.load(Ordering::Acquire)
            && driver_two.load(Ordering::Relaxed) == conn.conn_handle()
        {
            driver_two.store(0, Ordering::Relaxed);
            driver_connected_two.store(false, Ordering::Release);
        }
    });
    let service = server.create_service(uuid128!(GIX_SERVICE));
    // A static characteristic.
    driver_price_characteristic
        .lock()
        .on_read(move |args, _| {
            args.set_value(&[0; 16]);
        })
        .on_write(move |args| {
            let data = args.recv_data();
            if driver_connected_one.load(Ordering::Acquire) {
                log::info!("wrote to driver");
                if driver_one.load(Ordering::Relaxed) == args.desc().conn_handle() {
                    log::info!("wrote to driver connected and passed");
                    if data == CANCEL_CODE {
                        keypad_two.cancel.store(true, Ordering::Relaxed);
                        if let Ok(mut can_type) = keypad_two.can_type.lock() {
                            *can_type = false;
                            keypad_two.cond_var.notify_one();
                        }
                    } else {
                        let mut msg: heapless::Vec<char, 11> = heapless::Vec::new();
                        for c in data.iter() {
                            msg.push(*c as char).unwrap();
                        }
                        let mut has_msg_des = false;
                        if let Ok(has_msg) = keypad_two.has_msg.read() {
                            has_msg_des = *has_msg;
                        }
                        if let Ok(mut can_type) = keypad_two.can_type.lock() {
                            if *can_type && !has_msg_des {
                                log::info!("wrote to driver  and passed");
                                if let Ok(mut msg_write) = keypad_two.msg.write() {
                                    *msg_write = msg;
                                }
                                *can_type = false;
                                keypad_two.cond_var.notify_one();
                            }
                        }
                    }
                }
            }
            args.notify();
        });

    // A writagix characteristic.
    let writagix_characteristic = service.lock().create_characteristic(
    );
    writagix_characteristic
        .lock()
        .on_read(move |args, _| {
            args.set_value(&[0; 16]);
        })
        .on_write(move |args| {
            ::log::info!(
                args.current_data(),
                args.recv_data()
            );
            let data = args.recv_data();
            match confirm(data) {
                DeviceUser::Driver => {
                    driver.store(args.desc().conn_handle(), Ordering::Relaxed);
                    driver_connected.store(true, Ordering::Release);
                    log::info!("driver");
                }
                DeviceUser::User => {
                    conn_handle_price.store(args.desc().conn_handle(), Ordering::Relaxed);
                    conn_handle_device_id.store(args.desc().conn_handle(), Ordering::Relaxed);
                    connected_price.store(true, Ordering::Release);
                    connected_device_id.store(true, Ordering::Release);
                    log::info!("driviwr");
                }
                DeviceUser::None => {
                    log::info!("not verified");
                }
            }
            args.notify();
        });

    gix_advertising.lock().set_data(
        GIXAdvertisementData::new()
    )?;
    gix_advertising.lock().start()?;

    // let mut buf = [0_u8; 10];
    // let mut initialized = true;
    let mut found_disconnect: bool;
    let duration = Duration::from_secs(10);
    loop {
        found_disconnect = false;
        if let Ok(all_cons) = conns_arc.lock() {
            for conn in all_cons.iter() {
                log::info!("interval: {:?} {}", conn.time.elapsed(), conn.id);
                if conn.time.elapsed() > duration && conn.id != driver_three.load(Ordering::Relaxed)
                {
                    found_disconnect = true;
                    // Before we disconnect we check to see if it is still connected
                    if server
                        .connections()
                        .any(|desc| desc.conn_handle() == conn.id)
                    {
                        match server.disconnect(conn.id) {
                            Ok(_) => {
                                ::log::info!("disconnected");
                                // connected.store(false, Ordering::Relaxed);
                            }
                            Err(e) => {
                                ::log::error!("Error while disconnecting: {}", e);
                            }
                        }
                    }
                    // because there might be a possiblity that conn might not be ready yet
                    // We store the onces that were deleted
                }
            }
        }
        if found_disconnect {
            FreeRtos::delay_ms(1000);
        } else {
            FreeRtos::delay_ms(500);
        }
    }
}
fn contains_conn(my_conns: &heapless::Vec<Conn, 40>, conn: &u16) -> (bool, usize) {
    for (index, c) in my_conns.iter().enumerate() {
        if c.id == *conn {
            return (true, index);
        }
    }
    return (false, 0);
}

fn get_msg(
    keypad: &Arc<KeypadState>,
    connected: &AtomicBool,
    conn: &u16,
    connected_conn: &AtomicU16,
) -> [u8; 16] {
    let mut msg: [u8; 16] = ['a' as u8; 16];
    log::info!("get message id");
    let has_msg = match keypad.has_msg.read() {
        Ok(h) => *h,
        Err(_) => false,
    };
    if has_msg {
        if connected.load(Ordering::Acquire) {
            log::info!("connected good msg");
            if connected_conn.load(Ordering::Relaxed) == *conn {
                log::info!("== good msg");
                if let Ok(value) = keypad.msg.try_read().as_ref() {
                    for v in value.iter().enumerate() {
                        msg[v.0] = *v.1 as u8;
                    }
                }
                msg = aes_cipher(msg, ENCRYPTION_KEY_PRICE);
                connected.store(false, Ordering::Relaxed);
                connected_conn.store(0, Ordering::Release);
                keypad.sent.store(true, Ordering::Relaxed);
                if let Ok(mut has_msg) = keypad.has_msg.write() {
                    *has_msg = false;
                }
                if let Ok(mut can_type) = keypad.can_type.lock() {
                    *can_type = false;
                }
                keypad.cond_var.notify_one();
                log::info!("get message");
            }
        }
    }
    msg
}

fn get_device_id(connected: &AtomicBool, conn: &u16, connected_conn: &AtomicU16) -> [u8; 16] {
    let mut msg: [u8; 16] = [0; 16];
    log::info!("get message id");
    if connected
        .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        log::info!("good in device_id");
        if connected_conn.load(Ordering::Relaxed) == *conn {
            log::info!("good == in device_id");
            msg = aes_cipher(DEVICE_ID, ENCRYPTION_DEVICE_ID_KEY);
        }
        connected.store(false, Ordering::Relaxed);
        connected_conn.store(0, Ordering::Release);
        log::info!("get message id");
    }
    msg
}
