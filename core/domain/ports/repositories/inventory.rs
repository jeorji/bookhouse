use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait InventoryRepository:
    FindRepo<FindByBookIdDTO, InventoryDTO>
    + FindRepo<FindByWarehouseIdDTO, InventoryDTO>
    + CreateRepo<CreateInventoryDTO, InventoryDTO>
{
}

pub struct FindByBookIdDTO {
    pub book_edition_id: uuid::Uuid,
}

pub struct FindByWarehouseIdDTO {
    pub warehouse_id: uuid::Uuid,
}

pub struct CreateInventoryDTO {
    pub book_edition_id: uuid::Uuid,
    pub warehouse_id: uuid::Uuid,
    pub stock: u32,
}

pub struct InventoryDTO {
    pub book_edition_id: uuid::Uuid,
    pub warehouse_id: uuid::Uuid,
    pub stock: u32,
}
