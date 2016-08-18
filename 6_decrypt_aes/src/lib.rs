extern crate crypto;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub fn decrypt_aes_cbc(key: &[u8], cipher_data: &[u8]) -> Result<String, symmetriccipher::SymmetricCipherError> {
    let key_size_in_bytes = key.len();

    let iv = &cipher_data[..key_size_in_bytes];
    let encrypted_data = &cipher_data[key_size_in_bytes..];

    let key_size_in_bits = key_size_in_bytes * 8;
    let key_size_type = if key_size_in_bits == 128 {
        aes::KeySize::KeySize128
    } else if key_size_in_bits == 192 {
        aes::KeySize::KeySize192
    } else {
        aes::KeySize::KeySize256
    };

    let mut decryptor = aes::cbc_decryptor(key_size_type, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    let plain_text = final_result.into_iter().map(|x| x as char)
        .into_iter().collect::<String>();
    Ok(plain_text)
}

pub fn decrypt_aes_ctr(key: &[u8], cipher_data: &[u8]) -> String {
    let key_size_in_bytes = key.len();

    let iv = &cipher_data[..key_size_in_bytes];
    let encrypted_data = &cipher_data[key_size_in_bytes..];

    let key_size_in_bits = key_size_in_bytes * 8;
    let key_size_type = if key_size_in_bits == 128 {
        aes::KeySize::KeySize128
    } else if key_size_in_bits == 192 {
        aes::KeySize::KeySize192
    } else {
        aes::KeySize::KeySize256
    };

    let mut ctr = aes::ctr(key_size_type, key, iv);

    let mut result: Vec<u8> = vec![0; encrypted_data.len()];
    ctr.process(encrypted_data, &mut result);

    result.into_iter().map(|x| x as char)
        .into_iter().collect::<String>()
}


