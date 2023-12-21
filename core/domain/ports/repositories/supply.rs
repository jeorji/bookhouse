use super::CreateRepo;
use super::FindRepo;
use crate::entities::supply::SupplyStatus;

#[async_trait::async_trait]
pub trait SupplyRepository:
    FindRepo<FindByIdDTO, SupplyDTO> + CreateRepo<CreateSupplyDTO, SupplyDTO>
{
}

pub struct FindByIdDTO {
    pub id: uuid::Uuid,
}

pub struct CreateSupplyDTO {
    pub supplier_id: uuid::Uuid,
    pub status: SupplyStatus,
    pub warehouse_id: uuid::Uuid,
    pub date: u32,
}

pub struct SupplyDTO {
    pub id: uuid::Uuid,
    pub supplier_id: uuid::Uuid,
    pub status: SupplyStatus,
    pub warehouse_id: uuid::Uuid,
    pub date: u32,
}
