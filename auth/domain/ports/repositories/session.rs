use crate::ports::Error;

#[async_trait::async_trait]
pub trait SessionRepository {
    async fn find(&self, dto: FindSessionDTO) -> Result<Option<SessionDTO>, Error>;
    async fn update(&self, dto: UpdateSessionDTO) -> Result<bool, Error>;
    async fn create(&self, dto: CreateSessionDTO) -> Result<SessionDTO, Error>;
}
pub type BxSessionRepository = Box<dyn SessionRepository>;

pub struct FindSessionDTO {
    pub refresh_token: String,
}

pub struct UpdateSessionDTO {
    pub new_refresh_token: String,
}

pub struct CreateSessionDTO {
    pub user_id: uuid::Uuid,
    pub refresh_token: String,
}

pub struct SessionDTO {}
