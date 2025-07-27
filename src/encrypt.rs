use aes::{
    cipher::{
        consts::{B0, B1},
        generic_array::GenericArray,
        typenum::{UInt, UTerm},
        BlockDecrypt, BlockEncrypt,
    },
    Aes128,
};
use aes_gcm::KeyInit;

use crate::constant::{CONNECT_MSG, DRIVER_CONNECT_MSG, ENCRYPTION_KEY};
// use aes_gcm::{
//     aead::{Aead, AeadCore, OsRng}, Aes256Gcm, Key, KeyInit, Nonce // Or `Aes128Gcm`
// };
// aes_decipher means it uses the Aes128 algorithm to decrypt (16 bits long)
pub fn aes_decipher(
    ciphertext: [u8; 16],
    key: [u8; 16],
) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>> {
    let mut block = GenericArray::from(ciphertext);
    let key = GenericArray::from(key);
    // Initialize cipher
    let cipher = Aes128::new(&key);
    cipher.decrypt_block(&mut block);
    block
}

pub fn basic_decipher(ciphertext: [u8; 16], key: [u8; 16]) -> [u8; 16] {
    let mut code: [u8; 16] = [0; 16];
    let block = aes_decipher(ciphertext, key);
    for i in block.iter().enumerate() {
        code[i.0] = *i.1;
    }
    code
}
// basic_cipher means it uses the Aes128 algorithm to encrypt (16 bits long)
pub fn aes_cipher(price: [u8; 16], price_key: [u8; 16]) -> [u8; 16] {
    let mut code: [u8; 16] = [0; 16];
    let mut block = GenericArray::from(price);
    let key = GenericArray::from(price_key);
    // Initialize cipher
    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut block);
    for i in block.iter().enumerate() {
        code[i.0] = *i.1;
    }
    return code;
}
// pub fn encrypt_device_id(data: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
//     let key = Key::<Aes256Gcm>::from_slice(key);
//     let cipher = Aes256Gcm::new(key);

//     let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
//     let ciphertext = cipher.encrypt(&nonce, data)?;

//     let mut result = nonce.to_vec();
//     result.extend_from_slice(&ciphertext);

//     Ok(result)
// }

// pub fn decrypt_device_id(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
//     let key = Key::<Aes256Gcm>::from_slice(key);
//     let cipher = Aes256Gcm::new(key);

//     let (nonce, ciphertext) = encrypted_data.split_at(12);
//     let nonce = Nonce::from_slice(nonce);

//     cipher.decrypt(nonce, ciphertext)
// }

pub fn confirm(plaintext: &[u8]) -> DeviceUser {
    let mut block = [0; 16];
    for i in plaintext.iter().enumerate() {
        block[i.0] = *i.1;
    }
    block = basic_decipher(block, ENCRYPTION_KEY);
    if CONNECT_MSG == block[4..13] {
        return DeviceUser::User;
    } else if DRIVER_CONNECT_MSG == block[4..13] {
        return DeviceUser::Driver;
    }
    return DeviceUser::None;
}

pub enum DeviceUser {
    Driver,
    User,
    None,
}
