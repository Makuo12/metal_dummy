

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

pub enum GET {
    Health
}

use serde::{de::DeserializeOwned, Deserialize, Serialize};


pub fn get<T>(request_type: GET) -> Result<T> where T: DeserializeOwned {
    // 1. Create a new EspHttpClient. (Check documentation)
    // ANCHOR: connection
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let url = match request_type {
        GET::Health => GET_HEALTH_URL,
    };
    // ANCHOR_END: connection
    let mut client = Client::wrap(connection);
    
    // 2. Open a GET request to `url`
    let headers = [("accept", "application/json"), (XMINISTER_METAL_API_KEY, APK_KEY)];
    // Above we add the api key
    let request = client.request(Method::Get, url.as_ref(), &headers)?;

    // 3. Submit write request and check the status code of the response.
    // Successful http status codes are in the 200..=299 range.
    let response = request.submit()?;
    let status = response.status();
    let mut values = String::new();
    match status {
        200..=299 => {
            let mut buf = [0_u8; 256];
            let mut offset = 0;
            let mut total = 0;
            let mut reader = response;
            loop {
                // Here we parse the data...
            }
            println!("Total: {} bytes", total);
        }
        _ => bail!("Unexpected response code: {}", status),
    }
    let parsed = serde_json::from_str(&values)?;
    Ok(parsed)
}

pub fn post_request<T>(payload: &String) -> anyhow::Result<T> where T: DeserializeOwned {
    let connection = EspHttpConnection::new(&Configuration {
    use_global_ca_store: true,
    crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
    ..Default::default()
    })?;
    let headers = [
        ("content-type", "application/json"),
        ("content-length", &*payload.len().to_string()),
        (XMINISTER_METAL_API_KEY, APK_KEY)
    ];
    let url = "https://ghost.flizzup-server.com/api/metal/payment";
    let mut client = Client::wrap(connection);
    let mut request = client.request(Method::Post, url.as_ref(), &headers)?;
    // Send request
    request.write_all(&payload.as_bytes())?;
    request.flush()?;
    info!("-> POST {url}");
    let response = request.submit()?;
    // Process response
    let status = response.status();
    info!("<- {status}");
    let mut values = String::new();
    match status {
        200..=299 => {
            // 4. if the status is OK, read response data chunk by chunk into a buffer and print it until done
            //
            // NB. see http_client.rs for an explanation of the offset mechanism for handling chunks that are
            // split in the middle of valid UTF-8 sequences. This case is encountered a lot with the given
            // example URL.
            let mut buf = [0_u8; 256];
            let mut offset = 0;
            let mut total = 0;
            let mut reader = response;
            loop {
                if let Ok(size) = Read::read(&mut reader, &mut buf[offset..]) {
                    if size == 0 {
                        break;
                    }
                    total += size;
                    // 5. try converting the bytes into a Rust (UTF-8) string and print it
                    let size_plus_offset = size + offset;
                    match str::from_utf8(&buf[..size_plus_offset]) {
                        Ok(text) => {
                            print!("{}", text);
                            values.push_str(text);
                            offset = 0;
                        }
                        Err(error) => {
                            let valid_up_to = error.valid_up_to();
                            unsafe {
                                print!("{}", str::from_utf8_unchecked(&buf[..valid_up_to]));
                            }
                            buf.copy_within(valid_up_to.., 0);
                            offset = size_plus_offset - valid_up_to;
                        }
                    }
                }
            }
            println!("Total: {} bytes", total);
        }
        _ => bail!("Unexpected response code: {}", status),
    }
    let parsed: T = serde_json::from_str(&values)?;
    Ok(parsed)
}
