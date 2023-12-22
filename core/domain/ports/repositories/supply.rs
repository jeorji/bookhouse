use super::CreateRepo;
use super::FindRepo;
use super::UpdateRepo;
use crate::entities::supply::SupplyStatus;

#[async_trait::async_trait]
pub trait SupplyRepository:
    FindRepo<FindByIdDTO, SupplyDTO>
    + FindRepo<FindByStatusDTO, Vec<SupplyDTO>>
    + CreateRepo<CreateSupplyDTO, SupplyDTO>
    + UpdateRepo<UpdateSupplyDTO, SupplyDTO>
{
}

pub struct FindByIdDTO {
    pub id: uuid::Uuid,
}
pub struct FindByStatusDTO {
    pub status: SupplyStatus,
}

pub struct CreateSupplyDTO {
    pub supplier_id: uuid::Uuid,
    pub status: SupplyStatus,
    pub warehouse_id: Option<uuid::Uuid>,
    pub date: Option<u32>,
}

pub struct UpdateSupplyDTO {
    pub id: uuid::Uuid,
    pub status: SupplyStatus,
    pub warehouse_id: Option<uuid::Uuid>,
    pub date: Option<u32>,
}

pub struct SupplyDTO {
    pub id: uuid::Uuid,
    pub supplier_id: uuid::Uuid,
    pub status: SupplyStatus,
    pub warehouse_id: Option<uuid::Uuid>,
    pub date: Option<u32>,
}
