use std::sync::{atomic::Ordering, Arc};

use esp_idf_svc::hal::{
    delay::{Delay, FreeRtos},
    i2c::I2cDriver,
};
use lcd1602_diver::{data_bus::I2CBus, LCD1602};

use crate::{
    lcd::display::{setup_customer_msg, show_message},
    state::KeypadState,
};

use super::display::{setup_lcd, setup_transmisson_msg};

pub fn lcd_connect(mut lcd: LCD1602<I2CBus<I2cDriver<'_>>>, keypad_state: Arc<KeypadState>) {
    let mut delay: Delay = Default::default();
    lcd.clear(&mut delay).unwrap();
    let mut can_type = keypad_state.can_type.lock().unwrap();
    setup_lcd(&mut lcd, &mut delay);
    loop {
        if *can_type {
            can_type = keypad_state.cond_var.wait(can_type).unwrap();
        } else {
            if keypad_state
                .cancel
                .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                lcd.clear(&mut delay).unwrap();
                setup_lcd(&mut lcd, &mut delay);
                if let Ok(mut has_msg) = keypad_state.has_msg.write() {
                    *has_msg = false;
                }
                *can_type = true;
            } else if keypad_state
                .sent
                .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                lcd.clear(&mut delay).unwrap();
                setup_transmisson_msg(&mut lcd, &mut delay);
                *can_type = true;
            } else {
                if let Ok(msg) = keypad_state.msg.try_write().as_mut() {
                    setup_customer_msg(&mut lcd, &mut delay);
                    show_message(&mut lcd, &mut delay, msg);
                    let mut msg_clone: [u8; 16] = [b'a'; 16];
                    for c in msg.iter().enumerate() {
                        msg_clone[c.0] = *c.1 as u8;
                    }
                    *can_type = true;

                    if let Ok(mut has_msg) = keypad_state.has_msg.write() {
                        *has_msg = true;
                    }
                }
            }
        }
        FreeRtos::delay_ms(2);
    }
}
