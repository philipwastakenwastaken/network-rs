mod client;

use client::{Client};
use std::io::{self};
use openssl::rsa::{Rsa, Padding};
use std::fs;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    loop {
        let client = Client::connect();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        buffer.pop();
        println!("Sending message {:?}", buffer);

        let pub_key = fs::read("./server_key.pem").unwrap();
        println!("Read public key...");

        let rsa_key = Rsa::public_key_from_pem(&pub_key).unwrap();
        let mut encrypted_data: Vec<u8> = vec![0; rsa_key.size() as usize];

        let _ = rsa_key.public_encrypt(buffer.as_bytes(), &mut encrypted_data, Padding::PKCS1).unwrap();
        println!("Encrypted message: {:?}", encrypted_data);
        println!("Size: {}", encrypted_data.len());

        client.write_message(&encrypted_data)?;
    }
}
