use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use anyhow::Result;

use common::keys::aes::AesKey;
use common::keys::key::{PublicKey, SymmetricKey};
use common::keys::rsa::{RsaPrivateKey, RsaPublicKey};
use common::transaction::{SignedTransaction, Transaction};

pub struct Client {
    stream: TcpStream,
    pk: RsaPublicKey,
    sk: RsaPrivateKey,
}

impl Client {
    pub fn connect(addr: &str, pk_pem_path: &str, sk_pem_path: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        println!("Connected!");

        Ok(Client {
            stream,
            pk: RsaPublicKey::from_pem(pk_pem_path),
            sk: RsaPrivateKey::from_pem(sk_pem_path),
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
    pub fn write_message<KeyType>(&mut self, msg: &[u8], key: &KeyType) -> std::io::Result<()>
    where
        KeyType: SymmetricKey,
    {
        let encrypted_msg = key.encrypt(msg).unwrap();
        self.stream.write(&encrypted_msg)?;
        Ok(())
    }

    pub fn send_transaction<KeyType>(
        &mut self,
        tx: Transaction,
        key: &KeyType,
    ) -> std::io::Result<()>
    where
        KeyType: SymmetricKey,
    {
        let stx = SignedTransaction::new(tx);
        let de_stx = bincode::serialize(&stx).unwrap();

        let encrypted_payload = key.encrypt(&de_stx).unwrap();
        self.stream.write(&encrypted_payload)?;
        Ok(())
    }
}
