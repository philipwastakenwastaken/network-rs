use std::net::{TcpStream};
use std::io::{Write};
use std::str;

const LISTEN_ADDR: &str = "127.0.0.1:3333";

pub struct Client {
    stream: TcpStream
}


impl Client {

    pub fn connect() -> Self {
        let stream = TcpStream::connect(LISTEN_ADDR);
        assert!(stream.is_ok(), "Could not connect");
        println!("Connected!");

        Client {
            stream: stream.unwrap()
        }
    }

    pub fn write_message(mut self, msg: &Vec<u8>) -> std::io::Result<()> {
        self.stream.write(msg)?;
        Ok(())
    }



}