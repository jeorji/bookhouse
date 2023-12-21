use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait WarehouseRepository:
    FindRepo<FindByIdDTO, WarehouseDTO> + CreateRepo<CreateWarehouseDTO, WarehouseDTO>
{
}

pub struct FindByIdDTO {
    pub id: uuid::Uuid,
}

pub struct CreateWarehouseDTO {
    pub name: String,
    pub capacity: u32,
    pub location: (f64, f64),
}

pub struct WarehouseDTO {
    pub id: uuid::Uuid,
    pub name: String,
    pub capacity: u32,
    pub location: (f64, f64),
}
