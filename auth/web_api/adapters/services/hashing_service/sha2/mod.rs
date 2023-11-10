use auth::ports::{
    services::hashing_service::HashingService,
    services::hashing_service::{HashPasswordDTO, VerifyPasswordDTO},
};
use sha2::{Digest, Sha256};

pub struct Sha256Hasher {}

const SALT_LENGHT: usize = 16;

impl HashingService for Sha256Hasher {
    fn hash(&self, dto: HashPasswordDTO) -> (String, String) {
        let salt = hash_with_sha256(&generate_salt(SALT_LENGHT));
        let salted_password = format!("{}{}", dto.plain_password, salt);
        let hash = hash_with_sha256(salted_password.as_bytes());
        (hash, salt)
    }

    fn verify(&self, dto: VerifyPasswordDTO) -> bool {
        let salted_password = format!("{}{}", dto.plain_password, dto.salt);
        let hash = hash_with_sha256(salted_password.as_bytes());
        hash == dto.password_hash
    }
}

fn hash_with_sha256(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    format!("{:x}", hash)
}

fn generate_salt(length: usize) -> Vec<u8> {
    use rand::Rng;

    let mut salt = vec![0u8; length];
    rand::thread_rng().fill(&mut salt[..]);
    salt
}
