use std::io::Write;
use std::net::TcpStream;
use std::str;

use anyhow::Result;

use common::keys::key::PublicKey;
use common::keys::rsa::RsaPublicKey;

pub struct Client {
    stream: TcpStream,
    pk: RsaPublicKey,
}

impl Client {
    pub fn connect(addr: &str, pk_pem_path: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        println!("Connected!");

        Ok(Client {
            stream: stream,
            pk: RsaPublicKey::from_pem(pk_pem_path),
        })
    }

    pub fn write_message(mut self, msg: &[u8]) -> std::io::Result<()> {
        let encrypted_msg = self.pk.encrypt(msg);
        self.stream.write(&encrypted_msg)?;
        Ok(())
    }
}
