use anyhow::Result;

pub trait PrivateKey {
    fn decrypt(&self, msg: &[u8]) -> Vec<u8>;

    fn sign(&self) -> Vec<u8>;
}

pub trait PublicKey {
    fn encrypt(&self, msg: &[u8]) -> Vec<u8>;
}

pub trait SymmetricKey {
    fn encrypt(&self, msg: &[u8]) -> Result<Vec<u8>>;

    fn decrypt(&self, msg: &[u8]) -> Result<Vec<u8>>;
}
