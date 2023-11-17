use crate::ports::Error;

pub trait TokenService {
    fn issue(&self, dto: IssueTokenDTO) -> Result<TokenPairDTO, Error>;
    fn verify(&self, dto: VerifyTokenDTO) -> Result<bool, Error>;
    fn renew(&self, dto: RenewTokenDTO) -> Result<TokenPairDTO, Error>;
}
pub type BxTokenService = Box<dyn TokenService>;

pub struct IssueTokenDTO {
    pub user_id: uuid::Uuid,
    pub permission_group: i32,
}

pub struct VerifyTokenDTO {
    pub token: String,
}

pub struct RenewTokenDTO {
    pub token: String,
    pub refresh_token: String,
}

pub struct TokenPairDTO {
    pub token: String,
    pub refresh_token: String,
}
