use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
//use std::thread;
use std::str;

use common::keys::key::{PrivateKey, PublicKey, SymmetricKey};
use common::keys::rsa::{RsaPrivateKey, RsaPublicKey};
use common::keys::aes::AesKey;


pub struct Server {
    listener: TcpListener,
    sk: RsaPrivateKey,
    pk: RsaPublicKey
}

impl Server {
    pub fn bind(addr: &str, sk_pem_path: &str, pk_pem_path: &str) -> Server {
        let listener = TcpListener::bind(addr);
        assert!(listener.is_ok(), "Could not bind to address");
        println!("Connected to {}", addr);
        Server {
            listener: listener.unwrap(),
            sk: RsaPrivateKey::from_pem(sk_pem_path),
            pk: RsaPublicKey::from_pem(pk_pem_path)
        }
    }

    pub fn exchange_public_keys(&self, mut stream: &TcpStream) -> RsaPublicKey {
        let mut buffer = [0 as u8; 1024];
        let pk_pem = self.pk.to_pem();

        // Exchange public keys
        println!("Receiving client pk...");
        let pk_pem_client_len = stream.read(&mut buffer).unwrap();
        println!("Received client pk: {}", pk_pem_client_len);
        let pk_pem_client = &buffer[0..pk_pem_client_len];

        println!("Sending own pk...");
        stream.write(&pk_pem).unwrap();

        println!("Handshake done!");

        RsaPublicKey::from_pem_bin_data(pk_pem_client)
    }

    fn receive_session_key(&self, mut stream: &TcpStream) -> AesKey {
        let mut buffer = [0 as u8; 1024];
        let key_len = stream.read(&mut buffer).unwrap();
        let dec_key = self.sk.decrypt(&buffer[0..key_len]);

        println!("Received session key: {:?}", dec_key);

        bincode::deserialize(&dec_key).unwrap()
    }

    fn handle_connection(&self, mut stream: &TcpStream, aes_key: &AesKey) {
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

                let decrypted_msg = aes_key.decrypt(&buf[0..size]).unwrap();
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
                Ok(mut stream) => {
                    println!("Received connection: {}", stream.peer_addr().unwrap());

                    // Do initial setup to secure connection
                    self.exchange_public_keys(&mut stream);

                    let aes_key = self.receive_session_key(&mut stream);
                    debug_assert_eq!(aes_key.decrypt(&aes_key.encrypt("hi".as_bytes()).unwrap()).unwrap(), "hi".as_bytes());

                    // Send actual payloa
                    self.handle_connection(&stream, &aes_key);
                    println!("Connection ended");
                }

                Err(e) => {
                    println!("Connection failed! {}", e);
                }
            }
        }
    }
}
