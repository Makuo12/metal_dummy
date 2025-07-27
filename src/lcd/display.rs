use esp_idf_svc::hal::{delay::Delay, i2c::I2cDriver};
use heapless::Vec;
use lcd1602_diver::{data_bus::I2CBus, LCD1602};

pub fn show_message(
    lcd: &mut LCD1602<I2CBus<I2cDriver<'_>>>,
    delay: &mut Delay,
    msg: &mut Vec<char, 11>,
) {
    let mut new_msg: Vec<char, 14> = Vec::new();
    msg.reverse();
    for (index, c) in msg.iter().enumerate() {
        if (index % 3) == 0 && index != 0 {
            new_msg.push(',').unwrap();
        }
        new_msg.push(*c).unwrap();
    }
    msg.reverse();
    new_msg.reverse();
    for c in new_msg {
        lcd.write_char(c, delay).unwrap();
    }
}

pub fn setup_customer_msg(lcd: &mut LCD1602<I2CBus<I2cDriver<'_>>>, delay: &mut Delay) {
    lcd.clear(delay).unwrap();
    lcd.write_char('T', delay).unwrap();
    lcd.write_char('o', delay).unwrap();
    lcd.write_char('t', delay).unwrap();
    lcd.write_char('a', delay).unwrap();
    lcd.write_char('l', delay).unwrap();
    lcd.write_char(' ', delay).unwrap();
    lcd.write_char('P', delay).unwrap();
    lcd.write_char('r', delay).unwrap();
    lcd.write_char('i', delay).unwrap();
    lcd.write_char('c', delay).unwrap();
    lcd.write_char('e', delay).unwrap();
    lcd.write_char(':', delay).unwrap();
    lcd.set_cursor_pos(40, delay).unwrap();
    lcd.write_char('N', delay).unwrap();
}

pub fn setup_lcd(lcd: &mut LCD1602<I2CBus<I2cDriver<'_>>>, delay: &mut Delay) {
    lcd.clear(delay).unwrap();
    lcd.write_char('E', delay).unwrap();
    lcd.write_char('N', delay).unwrap();
    lcd.write_char('T', delay).unwrap();
    lcd.write_char('E', delay).unwrap();
    lcd.write_char('R', delay).unwrap();
    lcd.write_char(' ', delay).unwrap();
    lcd.write_char('P', delay).unwrap();
    lcd.write_char('R', delay).unwrap();
    lcd.write_char('I', delay).unwrap();
    lcd.write_char('C', delay).unwrap();
    lcd.write_char('E', delay).unwrap();
    lcd.write_char(':', delay).unwrap();
    lcd.set_cursor_pos(40, delay).unwrap();
    lcd.write_char('N', delay).unwrap();
}

pub fn setup_transmisson_msg(lcd: &mut LCD1602<I2CBus<I2cDriver<'_>>>, delay: &mut Delay) {
    lcd.clear(delay).unwrap();
    lcd.write_char('P', delay).unwrap();
    lcd.write_char('a', delay).unwrap();
    lcd.write_char('y', delay).unwrap();
    lcd.write_char('m', delay).unwrap();
    lcd.write_char('e', delay).unwrap();
    lcd.write_char('n', delay).unwrap();
    lcd.write_char('t', delay).unwrap();
    lcd.write_char(' ', delay).unwrap();
    lcd.write_char('i', delay).unwrap();
    lcd.write_char('n', delay).unwrap();
    lcd.write_char(',', delay).unwrap();
    lcd.set_cursor_pos(40, delay).unwrap();
    lcd.write_char('p', delay).unwrap();
    lcd.write_char('r', delay).unwrap();
    lcd.write_char('o', delay).unwrap();
    lcd.write_char('g', delay).unwrap();
    lcd.write_char('r', delay).unwrap();
    lcd.write_char('e', delay).unwrap();
    lcd.write_char('s', delay).unwrap();
    lcd.write_char('s', delay).unwrap();
}
