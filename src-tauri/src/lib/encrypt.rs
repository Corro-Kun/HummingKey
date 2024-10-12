use aes::cipher::{generic_array::GenericArray, KeyInit};
use aes::cipher::{BlockDecrypt, BlockEncrypt};
use aes::Aes256;
use hex::{decode, encode};
use block_padding::generic_array::functional::FunctionalSequence;


pub fn new_key(password: &String) -> Aes256 {
    let mut key = GenericArray::from([0u8; 32]);
    let password_bytes = password.as_bytes();
    let mut index = 0;

    key = key.map(|mut x|{
        if password_bytes.len() > index{
            x = password_bytes[index];
            index += 1;
        }
        x
    });

    let cipher = Aes256::new(&key);

    cipher
}

pub fn encrypt(data: &String, cipher: &Aes256) -> String{
    let data_bytes = data.as_bytes();
    let mut index = 0;
    let mut data_encrypt = String::new();

    while index < data_bytes.len(){
        let mut block = GenericArray::from([0u8; 16]);
        let end = std::cmp::min(index + 16, data_bytes.len());
        let length = end - index;

        block[..length].copy_from_slice(&data_bytes[index..end]);

        cipher.encrypt_block(&mut block);

        data_encrypt = data_encrypt + &encode(block).to_string();

        index += 16;
    }

    data_encrypt
}

pub fn decrypt(data_encrypt: &String, cipher: &Aes256) -> String{
    let mut index = 0;
    let mut data = String::new();

    while index < data_encrypt.len(){
        let mut block = GenericArray::from([0u8; 16]);
        let end = std::cmp::min(index + 32, data_encrypt.len());

        let decode = decode(&data_encrypt[index..end]).expect("Error decoding");

        block[..16].clone_from_slice(&decode);

        cipher.decrypt_block(&mut block);

        data = data + &String::from_utf8_lossy(&block);

        index += 32;
    }

    data
}