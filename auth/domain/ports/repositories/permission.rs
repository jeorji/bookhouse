use crate::ports::Error;

#[async_trait::async_trait]
pub trait PermissionRepository {
    async fn find(&self, dto: FindPermissionDTO) -> Result<Option<PermissionDTO>, Error>;
}
pub type BxPermissionRepository = Box<dyn PermissionRepository>;

pub struct FindPermissionDTO {
    pub user_id: uuid::Uuid,
}

pub struct PermissionDTO {
    pub user_id: uuid::Uuid,
    pub group: i32,
}
