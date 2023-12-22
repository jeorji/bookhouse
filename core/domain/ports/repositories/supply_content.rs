use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait SupplyContentRepository:
    FindRepo<FindBySupplyIdDTO, Vec<SupplyDTO>> + CreateRepo<CreateSupplyDTO, SupplyDTO>
{
}

pub struct FindBySupplyIdDTO {
    pub supply_id: uuid::Uuid,
}

pub struct CreateSupplyDTO {
    pub supply_id: uuid::Uuid,
    pub book_id: uuid::Uuid,
    pub quanity: u32,
    pub price: u32,
}

pub struct SupplyDTO {
    pub supply_id: uuid::Uuid,
    pub book_id: uuid::Uuid,
    pub quanity: u32,
    pub price: u32,
}
