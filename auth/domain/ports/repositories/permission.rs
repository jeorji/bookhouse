use crate::ports::Error;

#[async_trait::async_trait]
pub trait PermissionRepository {
    async fn find(&self, dto: FindPermissionDTO) -> Result<Option<PermissionDTO>, Error>;
    async fn create(&self, dto: CreatePermissionDTO) -> Result<PermissionDTO, Error>;
}

pub struct FindPermissionDTO {
    pub user_id: uuid::Uuid,
}

pub struct CreatePermissionDTO {
    pub user_id: uuid::Uuid,
    pub group: i32,
}

pub struct PermissionDTO {
    pub user_id: uuid::Uuid,
    pub group: i32,
}
