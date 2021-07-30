use crate::keys::key::SymmetricKey;
use anyhow::Result;
use openssl::rand::rand_bytes;
use openssl::symm::{encrypt, decrypt, Cipher};
use serde::{Serialize, Deserialize};

const N_BITS: usize = 256;
const N_BYTES: usize = N_BITS / 8;
const IV_BYTES: usize = 16;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct AesKey {
    key: [u8; N_BYTES],
    iv: [u8; IV_BYTES]
}

impl AesKey {
    pub fn new() -> Result<AesKey> {
        let mut key = [0 as u8; N_BYTES];
        let mut iv = [0 as u8; IV_BYTES];

        rand_bytes(&mut key)?;
        rand_bytes(&mut iv)?;

        Ok(AesKey {
            key,
            iv
        })
    }
}

impl SymmetricKey for AesKey {

    // Encrypt data using AES-256 with CBC padding.
    fn encrypt(&self, msg: &[u8]) -> Result<Vec<u8>> {
        let cipher = Cipher::aes_256_cbc();
        let enc_msg = encrypt(cipher, &self.key, Some(&self.iv), msg).unwrap();

        Ok(enc_msg.to_vec())
    }


    fn decrypt(&self, msg: &[u8]) -> Result<Vec<u8>> {
        let cipher = Cipher::aes_256_cbc();
        let dec_msg = decrypt(cipher, &self.key, Some(&self.iv), msg).unwrap();

        Ok(dec_msg)
    }
}

#[cfg(test)]
mod tests {
    use crate::keys::aes::AesKey;
    use crate::keys::key::SymmetricKey;

    #[test]
    fn encrypt_decrypt() {
        let msg = "Hi there!".as_bytes();

        let key = AesKey::new().unwrap();

        let enc_msg = key.encrypt(msg).unwrap();
        assert_ne!(msg, enc_msg);

        let dec_msg = key.decrypt(&enc_msg).unwrap();
        assert_eq!(msg, dec_msg);
    }

    #[test]
    fn serialize_deserialize() {
        let key = AesKey::new().unwrap();

        let key_serialized = bincode::serialize(&key).unwrap();

        let key_deserialized: AesKey = bincode::deserialize(&key_serialized).unwrap();

        assert_eq!(key, key_deserialized);

        let msg = "my guy!".as_bytes();

        let enc_msg1 = key.encrypt(msg).unwrap();
        let enc_msg2 = key_deserialized.encrypt(msg).unwrap();

        assert_eq!(enc_msg1, enc_msg2);
        assert_eq!(key_deserialized.decrypt(&enc_msg2).unwrap(), msg);
        assert_eq!(key_deserialized.decrypt(&enc_msg1).unwrap(), msg);
    }
}