// Testing was done using wifi
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
            let mut buf = [0_u8; 256];
            let mut offset = 0;
            let mut total = 0;
            let mut reader = response;
            loop {
                ...
            }
        }
        _ => bail!("Unexpected response code: {}", status),
    }
    let parsed: T = serde_json::from_str(&values)?;
    Ok(parsed)
}
