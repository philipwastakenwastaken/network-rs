use openssl::sha;

const SHA256_OUTPUT_BYTES: usize = 32;
pub type Sha256Hash = [u8; SHA256_OUTPUT_BYTES];

pub fn double_sha256(input: &[u8]) -> Sha256Hash {
    let single_hash = sha::sha256(&input);
    let double_hash = sha::sha256(&single_hash);
    double_hash
}