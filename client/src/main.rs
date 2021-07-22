mod client;

use client::{Client};
use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    loop {
        let client = Client::connect();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        println!("Received message {:?}", buffer);
        client.write_message(buffer)?;
    }
}
