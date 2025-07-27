use std::sync::Arc;

use gix::gix_dev;
use esp_idf_svc::{hal::{
    delay::Delay,
    gpio::{AnyIOPin, Level, PinDriver, Pull},
    i2c::{I2c, I2cConfig, I2cDriver},
    peripheral::Peripheral,
    prelude::Peripherals,
    units::{FromValueType, Hertz},
}, sys::esp_ota_abort};
use lcd::control::lcd_connect;
use lcd1602_diver::LCD1602;
use state::KeypadState;

mod gix;
mod constant;
mod encrypt;
mod lcd;
mod state;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    // into makes it possigix for us to convert the gpio pin for it normal state to a specific type.
    let i2c = i2c_master_init(
        peripherals.i2c0,
        peripherals.pins.gpio4.into(),
        peripherals.pins.gpio5.into(),
        100_u32.kHz().into(),
    )
    .unwrap();
    let mut delay: Delay = Default::default();
    let lcd = LCD1602::new_i2c(i2c, LCD_ADDRESS, &mut delay).unwrap();
    let keypad_state = Arc::new(KeypadState::new());

    let keypad_state_one = keypad_state.clone();
    let thread1 = std::thread::Builder::new()
        .stack_size(7000)
        .spawn(move || {
            lcd_connect(lcd, keypad_state_one);
        })
        .unwrap();
    let keypad_state_two = keypad_state.clone();
    gix_dev(keypad_state_two).unwrap();
    thread1.join().unwrap();
}

fn i2c_master_init<'d>(
    i2c: impl Peripheral<P = impl I2c> + 'd,
    sda: AnyIOPin,
    scl: AnyIOPin,
    baudrate: Hertz,
) -> anyhow::Result<I2cDriver<'d>> {
    let config = I2cConfig::new().baudrate(baudrate);
    let driver = I2cDriver::new(i2c, sda, scl, &config)?;
    Ok(driver)
}

const LCD_ADDRESS: u8 = 0x27;
