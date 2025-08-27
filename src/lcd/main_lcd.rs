use std::sync::Arc;

use esp_idf_svc::hal::{delay::Delay, i2c::I2cDriver};
use lcd1602_diver::{data_bus::I2CBus, Direction, LCD1602};

use crate::{lcd::{display::{network, payment_done, setup_customer_msg, setup_lcd, setup_transmisson_msg, show_message, write_message}, task::LCDCommand}, state::MainState};


pub fn lcd_display(mut lcd: LCD1602<I2CBus<I2cDriver<'_>>>, state: Arc<MainState>) {
    let mut delay: Delay = Default::default();
    loop {
        let mut cmd: Option<LCDCommand> = Option::None;
        if let Ok(mut guard) = state.lcd_command.try_lock() {
            cmd = guard.take();
            if cmd.is_none() {
                let _result = state.cond_var_lcd.wait(guard).unwrap();
                continue;
            }
        };
        if let Some(cmd) = cmd.take() {
            match cmd {
                super::task::LCDCommand::Message(_) => {
                    lcd.clear(&mut delay).unwrap();
                    setup_lcd(&mut lcd, &mut delay);
                },
                super::task::LCDCommand::SetupTransmissionMsg(on_sent) => {
                    if on_sent {
                        setup_lcd(&mut lcd, &mut delay);
                    }
                    setup_transmisson_msg(&mut lcd, &mut delay);
                },
                super::task::LCDCommand::HealthCheck((health, on_sent)) => {
                    if on_sent {
                        setup_lcd(&mut lcd, &mut delay);
                    }
                    network(&mut lcd, health.healthly, &mut delay);
                },
                super::task::LCDCommand::PaymentDone((_, data)) => {
                    payment_done(&mut lcd, &mut delay, &data.first_name)
                },
                super::task::LCDCommand::PreviousMsg(_) => {
                    setup_lcd(&mut lcd, &mut delay);
                    if let Ok(msg) = state.msg.try_write().as_mut() {
                        show_message(&mut lcd, &mut delay, msg);
                    }
                },
                super::task::LCDCommand::PriceEntered(on_sent) => {
                    if on_sent {
                        setup_lcd(&mut lcd, &mut delay);
                    }
                    setup_customer_msg(&mut lcd, &mut delay);
                    if let Ok(msg) = state.msg.try_write().as_mut() {
                        show_message(&mut lcd, &mut delay, msg);
                    }
                },
                super::task::LCDCommand::ClearLastChar(on_sent) => {
                    if on_sent {
                        setup_lcd(&mut lcd, &mut delay);
                    }
                    if let Ok(msg) = state.msg.try_write().as_mut() {
                        lcd.shift_cursor(Direction::Left, &mut delay).unwrap();
                        lcd.write_char(' ', &mut delay).unwrap();
                        lcd.shift_cursor(Direction::Left, &mut delay).unwrap();
                        msg.pop().unwrap();
                    }
                }, 
                super::task::LCDCommand::Character((c, on_sent)) => {
                    if on_sent {
                        setup_lcd(&mut lcd, &mut delay);
                    }
                    if let Ok(msg) = state.msg.try_write().as_mut() {
                        if msg.len() < 11 {
                            lcd.write_char(c, &mut delay).unwrap();
                        }
                    }
                }
            }
        }
    }
    
}