use std::sync::{atomic::Ordering, Arc};

use esp_idf_svc::hal::{
    delay::{Delay, FreeRtos},
    gpio::{Gpio0, Gpio1, Gpio18, Gpio19, Gpio2, Gpio3, Gpio6, Gpio7, Input, Output, PinDriver},
};

use crate::{keypad::Keypad4x4, lcd::task::{lcd_task, LCDCommand}, state::MainState};


pub fn keypad_control(
    state: Arc<MainState>,
    cols: (
        PinDriver<'_, Gpio3, Output>,
        PinDriver<'_, Gpio2, Output>,
        PinDriver<'_, Gpio1, Output>,
        PinDriver<'_, Gpio0, Output>,
    ),
    rows: (
        PinDriver<'_, Gpio19, Input>,
        PinDriver<'_, Gpio18, Input>,
        PinDriver<'_, Gpio7, Input>,
        PinDriver<'_, Gpio6, Input>,
    ),
) {
    let mut key = Keypad4x4::new(rows, cols);
    let delay: Delay = Default::default();
    let mut previous_msg: heapless::Vec<char, 11> = heapless::Vec::new();
    let mut random_msg: heapless::Vec<char, 11> = heapless::Vec::new();
    let mut can_type = state.can_type.lock().unwrap();
    let mut character = ' ';
    let mut on_sent = false;
    if let Ok(mut guard) = state.lcd_command.try_lock() {
        *guard = Some(LCDCommand::Message(on_sent));
            state.cond_var_lcd.notify_all();
    }
    loop {
        if !*can_type {
            can_type = state.cond_var.wait(can_type).unwrap();
        } else {
            if state.sent.compare_exchange(true, false,
                Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                    ...
            } else {
                ...
                if ch != ' ' {
                    if ch != character {
                        if let Ok(msg) = state.msg.try_write().as_mut() {
                            lcd_task(&mut on_sent, &state, ch, &mut previous_msg, msg, &mut can_type);
                        }
                        character = ch;
                    }
                } else {
                    character = ' ';
                }
            }
        }
        FreeRtos::delay_ms(2);
    }
}
