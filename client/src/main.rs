mod client;

use client::Client;
use common::constants::LISTEN_ADDR;
use std::io;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    loop {
        let mut client = Client::connect(LISTEN_ADDR, "./client_key.pem", "./private.pem").unwrap();
        let pk = client.exchange_public_keys();
        let aes_key = client.send_session_key(&pk);

        // Read input from console
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;

        // Removes new-line character
        buffer.pop();
        println!("Sending message {:?}", buffer);

        client.write_message(buffer.as_bytes(), &aes_key)?;
    }
}
