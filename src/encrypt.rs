use aes_gcm::{
    aead::{Aead}, Aes128Gcm, Key, KeyInit
};
use aes_gcm::Nonce as GcmNonce;

use crate::constant::MsgPipe;

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    // Key type changed to Aes128Gcm
    let key = Key::<Aes128Gcm>::from_slice(key);
    let cipher = Aes128Gcm::new(key);
    
    let nonce = GcmNonce::from_slice(&[0u8; 12]);
    let ciphertext = cipher.encrypt(&nonce, data)?;
    
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}


pub fn process_encrypt_msg(data: &MsgPipe) -> (String, String) {
    let mut id = String::new();
    let mut msg = String::new();
    for i in 0..data.len() {
        if i == 0 {
            let first = &data[i][4..12];
            let second = &data[i][12..];
            first.iter().for_each(|f| {
                id.push(*f as char);
            });
            id.push('-');
            second.iter().for_each(|f| {
                id.push(*f as char);
            });
        } else if i == 1 {
            id.push('-');
            let first = &data[i][3..7];
            let second = &data[i][7..11];
            first.iter().for_each(|f| {
                id.push(*f as char);
            });
            id.push('-');
            second.iter().for_each(|f| {
                id.push(*f as char);
            });
        } else if i == 14  {
            id.push('-');
            let first = &data[i][4..];
            first.iter().for_each(|f| {
                id.push(*f as char);
            });
        } else {
            if !data[i].is_empty() {
                data[i].iter().for_each(|f| {
                    msg.push(*f as char);
                });
            }
        }
    }
    return (id, msg);
}