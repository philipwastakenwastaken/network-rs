use std::io::{Write, Read};
use std::net::TcpStream;
use std::str;

use anyhow::Result;

use common::keys::key::{PublicKey, PrivateKey, SymmetricKey};
use common::keys::rsa::{RsaPublicKey, RsaPrivateKey};
use common::keys::aes::AesKey;
use common::transaction::{SignedTransaction, Transaction};


pub struct Client {
    stream: TcpStream,
    pk: RsaPublicKey,
    sk: RsaPrivateKey
}

impl Client {
    pub fn connect(addr: &str, pk_pem_path: &str, sk_pem_path: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        println!("Connected!");

        Ok(Client {
            stream,
            pk: RsaPublicKey::from_pem(pk_pem_path),
            sk: RsaPrivateKey::from_pem(sk_pem_path)
        })
    }

    pub fn exchange_public_keys(&mut self) -> RsaPublicKey {
        let mut buffer = [0 as u8; 1024];
        let pk_pem = self.pk.to_pem();

        // Exchange public keys
        self.stream.write(&pk_pem).unwrap();

        let pk_pem_server_len = self.stream.read(&mut buffer).unwrap();
        let pk_pem_server = &buffer[0..pk_pem_server_len];
        RsaPublicKey::from_pem_bin_data(pk_pem_server)
    }

    pub fn send_session_key(&mut self, server_pk: &RsaPublicKey) -> AesKey {
        let aes_key = AesKey::new().unwrap();

        let ser_aes_key = bincode::serialize(&aes_key).unwrap();
        println!("aes data {:?}", ser_aes_key);
        let enc_key = server_pk.encrypt(&ser_aes_key);
        self.stream.write(&enc_key).unwrap();

        aes_key
    }

    // Send a series of bytes through the verified channel using AES encryption.
    pub fn write_message(&mut self, msg: &[u8], aes_key: &AesKey) -> std::io::Result<()> {
        let encrypted_msg = aes_key.encrypt(msg).unwrap();
        self.stream.write(&encrypted_msg)?;
        Ok(())
    }

    pub fn send_transaction(&mut self, tx: Transaction, aes_key: &AesKey) -> std::io::Result<()> {
        let stx = SignedTransaction::new(tx);
        let de_stx = bincode::serialize(&stx).unwrap();

        let encrypted_payload = aes_key.encrypt(&de_stx).unwrap();
        self.stream.write(&encrypted_payload)?;
        Ok(())
    }
}
