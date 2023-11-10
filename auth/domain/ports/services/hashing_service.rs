pub trait HashingService {
    fn hash(&self, dto: HashPasswordDTO) -> (String, String);
    fn verify(&self, dto: VerifyPasswordDTO) -> bool;
}
pub type BxHashingService = Box<dyn HashingService>;

pub struct HashPasswordDTO {
    pub plain_password: String,
}

pub struct VerifyPasswordDTO {
    pub plain_password: String,
    pub salt: String,
    pub password_hash: String,
}
