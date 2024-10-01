use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, BlockDecrypt};
use aes::Aes256;
use hex::{decode, encode};

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