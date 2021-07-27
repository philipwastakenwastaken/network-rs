use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
//use std::thread;
use std::str;

use common::keys::key::PrivateKey;
use common::keys::rsa::RsaPrivateKey;

pub struct Server {
    listener: TcpListener,
    sk: RsaPrivateKey,
}

impl Server {
    pub fn bind(addr: &str, sk_pem_path: &str) -> Server {
        let listener = TcpListener::bind(addr);
        assert!(listener.is_ok(), "Could not bind to address");
        println!("Connected to {}", addr);
        Server {
            listener: listener.unwrap(),
            sk: RsaPrivateKey::from_pem(sk_pem_path),
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        const BUF_LEN: usize = 512;
        let mut buf = [0 as u8; BUF_LEN];

        println!("Reading messages..");
        match stream.read(&mut buf) {
            Ok(size) => {
                println!(
                    "Read stream of {} bytes: {:?}",
                    size,
                    str::from_utf8(&buf[0..size])
                );

                let decrypted_msg = self.sk.decrypt(&buf);
                println!("Msg: {:?}", str::from_utf8(&decrypted_msg));
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
