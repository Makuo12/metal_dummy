use std::sync::{Arc, MutexGuard};

use log::info;

use crate::{cloud::{client::{get, GET}, pipe::{Healthly, MetalPaymentResponse}}, state::MainState};


pub enum LCDCommand {
    Message(bool),
    SetupTransmissionMsg(bool),
    HealthCheck((Healthly, bool)), 
    PaymentDone((bool, MetalPaymentResponse)),
    PreviousMsg(bool),
    PriceEntered(bool),
    ClearLastChar(bool),
    Character((char, bool)),
}

pub fn lcd_task(
    on_sent: &mut bool,
    state: &Arc<MainState>,
    value: char,
    previous_msg: &mut heapless::Vec<char, 11>,
    msg: &mut heapless::Vec<char, 11>,
    can_type: &mut MutexGuard<'_, bool>
) {
    match value {
        'M' => {
            previous_msg.clear();
            for c in msg.iter() {
                previous_msg.push(*c).unwrap();
            }
            msg.clear();
            
            if let Ok(mut guard) = state.lcd_command.try_lock() {
                *guard = Some(LCDCommand::SetupTransmissionMsg(*on_sent));
                state.cond_var_lcd.notify_all();
            }
        }
        'A' => {
            println!("Checking health");
            if let Ok(data) = get::<Healthly>(GET::Health) {
                if let Ok(mut guard) = state.lcd_command.try_lock() {
                    *guard = Some(LCDCommand::HealthCheck((data, *on_sent)));
                    state.cond_var_lcd.notify_one();
                }
            }
        }
        'B' => {
            msg.clear();
            for c in previous_msg.iter() {
                msg.push(*c).unwrap();
            }
            if let Ok(mut guard) = state.lcd_command.try_lock() {
                *guard = Some(LCDCommand::PreviousMsg(*on_sent));
                state.cond_var_lcd.notify_one();
            }
        }
        'D' => {
            info!("[INFO] Price entered");
            if let Ok(mut guard) = state.lcd_command.try_lock() {
                *guard = Some(LCDCommand::PriceEntered(*on_sent));
                state.cond_var_lcd.notify_one();
            }
            let mut msg_clone: [u8; 16] = [b'a'; 16];
            for c in msg.iter().enumerate() {
                msg_clone[c.0] = *c.1 as u8;
            }
            **can_type = false;
        }
        'C' => {
            if !msg.is_empty() {
                if let Ok(mut guard) = state.lcd_command.try_lock() {
                    *guard = Some(LCDCommand::ClearLastChar(*on_sent));
                    state.cond_var_lcd.notify_one();
                }
            }
        }
        _ => {
            if msg.len() < 11 {
                let _ = msg.push(value);
                if let Ok(mut guard) = state.lcd_command.try_lock() {
                    *guard = Some(LCDCommand::Character((value, *on_sent)));
                    state.cond_var_lcd.notify_one();
                }
            }
        }
    }
    if *on_sent {
        *on_sent = false;
    }
}
