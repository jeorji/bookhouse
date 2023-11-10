use crate::ports::Error;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(&self, dto: CreateUserDTO) -> Result<UserDTO, Error>;
    async fn find(&self, email: FindUserDTO) -> Result<Option<UserDTO>, Error>;
}
pub type BxUserRepository = Box<dyn UserRepository>;

pub struct CreateUserDTO {
    pub email: String,
    pub password_hash: String,
    pub salt: String,
}

pub struct FindUserDTO {
    pub email: String,
}

pub struct UserDTO {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
}
