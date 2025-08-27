use std::sync::Arc;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetalPaymentRequest {
    customer_id: String,
    device_id: &'static str,
    pipe: String,
    encrypted_price: Vec<u8>,
    time: i64
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetalPaymentResponse {
    pub first_name: String,
    pub last_name: String,
    pub amount: i64,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Healthly {
    pub healthly: bool
}

pub fn pipe_stream(state: Arc<MainState>) {
    loop {
        let mut pipe_data = Option::None;
        if let Ok(mut data) = state.pipe.try_lock() {
            pipe_data = data.take();
            if pipe_data == Option::None {
                let _result = state.cond_var_pipe.wait(data);
            }
        }
        if let Some(pipe) = pipe_data {
                // We send the data to metal
                let (id, msg) = process_encrypt_msg(&pipe.0);
                if let Ok(price) = get_price(&state) {
                    let data = MetalPaymentRequest {
                        customer_id: id, 
                        device_id: DEVICE_ID,
                        pipe: msg,
                        encrypted_price: price,
                        time: pipe.1
                    };
                    if let Ok(payload) = serde_json::to_string_pretty(&data) {
                        if let Ok(result) = post_request::<MetalPaymentResponse>(&payload) {
                            if let Ok(mut guard) = state.lcd_command.try_lock() {
                                *guard = Some(LCDCommand::PaymentDone((false, result)));
                                state.cond_var_lcd.notify_all();
                            }
                            
                        } else {
                        // Handle error case
                        println!("Payment request failed");
                        }
                    } else {
                        ::log::info!("payload error happened could not serialize data");
                    }
                    
                }
        } else {
        }

    }
}