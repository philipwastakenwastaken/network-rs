use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
//use std::thread;
use openssl::rsa::{Padding, Rsa};
use std::fs;
use std::str;

pub const LISTEN_ADDR: &str = "127.0.0.1:3333";

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn bind(addr: &str) -> Server {
        let listener = TcpListener::bind(addr);
        assert!(listener.is_ok(), "Could not bind to address");
        println!("Connected to {}", addr);
        Server {
            listener: listener.unwrap(),
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        const BUF_LEN: usize = 512;
        let mut buf = [0 as u8; BUF_LEN];

        let sk = fs::read("./private.pem").unwrap();
        println!("Read private key...");

        let rsa_private_key = Rsa::private_key_from_pem(&sk).unwrap();

        let mut decrypted_message = [0 as u8; BUF_LEN];

        println!("Reading messages..");
        match stream.read(&mut buf) {
            Ok(size) => {
                println!(
                    "Read stream of {} bytes: {:?}",
                    size,
                    str::from_utf8(&buf[0..size])
                );
                let _ = rsa_private_key
                    .private_decrypt(&buf, &mut decrypted_message, Padding::PKCS1)
                    .unwrap();
                println!("Msg: {:?}", str::from_utf8(&decrypted_message[0..size]));
            }
            Err(_) => {
                println!("Error occurred, shutting down");
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }

    pub fn listen(self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Received connection: {}", stream.peer_addr().unwrap());
                    self.handle_connection(stream);
                    println!("Connection ended");
                }

                Err(e) => {
                    println!("Connection failed! {}", e);
                }
            }
        }
    }
}
