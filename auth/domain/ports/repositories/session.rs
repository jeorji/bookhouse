use crate::ports::Error;

#[async_trait::async_trait]
pub trait SessionRepository {
    async fn create(&self, dto: CreateSessionDTO) -> Result<SessionDTO, Error>;
}
pub type BxSessionRepository = Box<dyn SessionRepository>;

pub struct CreateSessionDTO {
    pub user_id: uuid::Uuid,
    pub refresh_token: String,
    pub refresh_token_exp: u64,
}

pub struct SessionDTO {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub refresh_token: String,
    pub refresh_token_exp: u64,
}
