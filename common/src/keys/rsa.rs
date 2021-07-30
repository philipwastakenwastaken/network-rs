use crate::keys::key::{PrivateKey, PublicKey};
use anyhow::Result;
use openssl::{
    pkey::{Private, Public},
    rsa::{Padding, Rsa},
};
use std::fs;

pub struct RsaPrivateKey {
    sk: Rsa<Private>,
}

pub struct RsaPublicKey {
    pk: Rsa<Public>,
}

pub struct RsaKeyPair {
    pub sk: RsaPrivateKey,
    pub pk: RsaPublicKey,
}

impl RsaKeyPair {
    pub fn new() -> Result<Self> {
        const N_BITS: u32 = 4096;

        // Our implementation expects public and private keys to separate.
        // However, openssl generates keys exclusively as a single merged pair.
        // In order to split the keys, we simply serialize each key to PEM-format
        // and deserialize.
        // TODO: feels a bit hacky, see if there is a better way.

        // Contains both public and private key.
        let key_pair = Rsa::generate(N_BITS)?;

        let pk_pem = key_pair.public_key_to_pem()?;
        let sk_pem = key_pair.private_key_to_pem()?;

        Ok(RsaKeyPair {
            sk: RsaPrivateKey::new(Rsa::private_key_from_pem(&sk_pem)?),
            pk: RsaPublicKey::new(Rsa::public_key_from_pem(&pk_pem)?),
        })
    }
}

impl RsaPrivateKey {
    pub fn from_pem(path: &str) -> Self {
        let sk = fs::read(path).unwrap();

        RsaPrivateKey {
            sk: Rsa::private_key_from_pem(&sk).unwrap(),
        }
    }

    fn new(key: Rsa<Private>) -> Self {
        RsaPrivateKey { sk: key }
    }

    pub fn to_pem(&self) -> Vec<u8> {
        self.sk.private_key_to_pem().unwrap()
    }
}

impl PrivateKey for RsaPrivateKey {
    fn decrypt(&self, msg: &[u8]) -> Vec<u8> {
        let mut decrypted_message = vec![0 as u8; self.sk.size() as usize];

        let size = self
            .sk
            .private_decrypt(&msg, &mut decrypted_message, Padding::PKCS1)
            .unwrap();

        decrypted_message[0..size].to_vec()
    }

    fn sign(&self) -> Vec<u8> {
        vec![1, 2, 3]
    }
}

impl RsaPublicKey {
    pub fn from_pem(path: &str) -> Self {
        let pk = fs::read(path).unwrap();

        RsaPublicKey {
            pk: Rsa::public_key_from_pem(&pk).unwrap(),
        }
    }

    pub fn from_pem_bin_data(pem: &[u8]) -> Self {

        RsaPublicKey {
            pk: Rsa::public_key_from_pem(&pem).unwrap(),
        }
    }

    fn new(key: Rsa<Public>) -> Self {
        RsaPublicKey { pk: key }
    }

    pub fn to_pem(&self) -> Vec<u8> {
        self.pk.public_key_to_pem().unwrap()
    }
}

impl PublicKey for RsaPublicKey {
    fn encrypt(&self, msg: &[u8]) -> Vec<u8> {
        let mut encrypted_data: Vec<u8> = vec![0; self.pk.size() as usize];

        let _ = self
            .pk
            .public_encrypt(&msg, &mut encrypted_data, Padding::PKCS1)
            .unwrap();
        encrypted_data
    }
}

#[cfg(test)]
mod tests {
    use crate::keys::{
        key::{PrivateKey, PublicKey},
        rsa::RsaKeyPair,
    };

    #[test]
    fn create_key_pair() {
        let _ = RsaKeyPair::new().unwrap();
    }

    #[test]
    fn encrypt_decrypt() {
        let kp = RsaKeyPair::new().unwrap();

        let msg = "my man!".as_bytes();

        let encrypted_msg = kp.pk.encrypt(&msg);
        // Make sure something actually happened.
        assert_ne!(msg, encrypted_msg);

        let decrypted_msg = kp.sk.decrypt(&encrypted_msg);
        assert_eq!(msg, decrypted_msg);
    }
}
