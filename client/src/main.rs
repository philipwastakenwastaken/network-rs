mod client;

use client::Client;
use std::io;
use common::constants::LISTEN_ADDR;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    loop {
        let client = Client::connect(LISTEN_ADDR, "./server_key.pem").unwrap();

        // Read input from console
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;

        // Removes new-line character
        buffer.pop();
        println!("Sending message {:?}", buffer);

        client.write_message(buffer.as_bytes())?;
    }
}
