extern crate rustc_serialize as serialize;
extern crate decrypt_aes;

use serialize::hex::FromHex;
use decrypt_aes::{ decrypt_aes_cbc, decrypt_aes_ctr };

const CBC_KEY_1: &'static str = "140b41b22a29beb4061bda66b6747e14";
const CBC_CIPHER_TEXT_1: &'static str = "4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81";
const CBC_KEY_2: &'static str = "140b41b22a29beb4061bda66b6747e14";
const CBC_CIPHER_TEXT_2: &'static str = "5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253";

const CTR_KEY_1: &'static str = "36f18357be4dbd77f050515c73fcf9f2";
const CTR_CIPHER_TEXT_1: &'static str = "69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329";
const CTR_KEY_2: &'static str = "36f18357be4dbd77f050515c73fcf9f2";
const CTR_CIPHER_TEXT_2: &'static str = "770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451";

fn main() {
    println!("{}", decrypt_aes_cbc(&CBC_KEY_1.from_hex().unwrap(),
                                   &CBC_CIPHER_TEXT_1.from_hex().unwrap()).unwrap());
    println!("{}", decrypt_aes_cbc(&CBC_KEY_2.from_hex().unwrap(),
                                   &CBC_CIPHER_TEXT_2.from_hex().unwrap()).unwrap());
    println!("{}", decrypt_aes_ctr(&CTR_KEY_1.from_hex().unwrap(),
                                   &CTR_CIPHER_TEXT_1.from_hex().unwrap()));
    println!("{}", decrypt_aes_ctr(&CTR_KEY_2.from_hex().unwrap(),
                                   &CTR_CIPHER_TEXT_2.from_hex().unwrap()));
}
