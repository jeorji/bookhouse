use crate::ports::Error;

#[async_trait::async_trait]
pub trait SessionRepository {
    async fn create(&self, dto: CreateSessionDTO) -> Result<SessionDTO, Error>;
    async fn update(&self, dto: UpdateSessionDTO) -> Result<Option<SessionDTO>, Error>;
    async fn find(&self, dto: FindSessionDTO) -> Result<Option<SessionDTO>, Error>;
}

pub struct CreateSessionDTO {
    pub user_id: uuid::Uuid,
    pub refresh_token: String,
    pub refresh_token_exp: u64,
}

pub struct UpdateSessionDTO {
    pub refresh_token: String,
    pub new_refresh_token: String,
    pub new_refresh_token_exp: u64,
}

pub struct FindSessionDTO {
    pub refresh_token: String,
}

pub struct SessionDTO {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub refresh_token: String,
    pub refresh_token_exp: u64,
}
