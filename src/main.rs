
fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    unsafe {
        let initialized = nvs_flash_init();
        if initialized == 0 {
            info!("NVS flash initialized successfully");
        } else {
            info!("NVS flash already initialized or error occurred");
        }
    }
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();

    let app_config = CONFIG;

    // Testing was done on wifi
    let wifi = wifi::wifi::wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    );
    if wifi.is_ok() {
    }
    let connected = AtomicBool::new(wifi.is_ok());
    let i2c = i2c_master_init(
        peripherals.i2c0,
        peripherals.pins.gpio4.into(),
        peripherals.pins.gpio5.into(),
        100_u32.kHz().into(),
    )
    .unwrap();
    let mut delay: Delay = Default::default();
    let lcd = LCD1602::new_i2c(i2c, LCD_ADDRESS, &mut delay).unwrap();
    let mut col_one = PinDriver::output_od(peripherals.pins.gpio0).unwrap();
    col_one.set_level(Level::High).unwrap();
    let mut col_two = PinDriver::output_od(peripherals.pins.gpio1).unwrap();
    col_two.set_level(Level::High).unwrap();
    let mut col_three = PinDriver::output_od(peripherals.pins.gpio2).unwrap();
    col_three.set_level(Level::High).unwrap();
    let mut col_four = PinDriver::output_od(peripherals.pins.gpio3).unwrap();
    col_four.set_level(Level::High).unwrap();

    let mut row_five = PinDriver::input(peripherals.pins.gpio6).unwrap();
    row_five.set_pull(Pull::Up).unwrap();
    let mut row_six = PinDriver::input(peripherals.pins.gpio7).unwrap();
    row_six.set_pull(Pull::Up).unwrap();
    let mut row_seven = PinDriver::input(peripherals.pins.gpio18).unwrap();
    row_seven.set_pull(Pull::Up).unwrap();
    let mut row_eight = PinDriver::input(peripherals.pins.gpio19).unwrap();
    row_eight.set_pull(Pull::Up).unwrap();

    let rows = (row_eight, row_seven, row_six, row_five);

    let cols = (col_four, col_three, col_two, col_one);
    let state = Arc::new(MainState::new(connected));

    let state_one = state.clone();
    let state_two = state.clone();
    let state_three = state.clone();
    let thread1 = std::thread::Builder::new()
        .stack_size(7000)
        .spawn(move || {
            keypad_control( state_one, cols, rows);
        })
        .unwrap();
    let thread2 = std::thread::Builder::new()
        .stack_size(7000)
        .spawn(move || {
            pipe_stream(state_two);
        })
        .unwrap();
    let thread3 = std::thread::Builder::new()
        .stack_size(7000)
        .spawn(move || {
            lcd_display(lcd, state_three);
        })
        .unwrap();
    let state_two = state.clone();
    jinx(state_two).unwrap();
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    loop {}
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
