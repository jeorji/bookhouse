use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait SupplierRepository:
    FindRepo<FindByIdDTO, SupplierDTO> + CreateRepo<CreateSupplierDTO, SupplierDTO>
{
}

pub struct FindByIdDTO {
    pub id: uuid::Uuid,
}

pub struct CreateSupplierDTO {
    pub user_id: uuid::Uuid,
    pub address_id: uuid::Uuid,
    pub street_number: String,
    pub apartment: String,
    pub name: String,
    pub bank: String,
    pub bank_account_number: String,
    pub inn: String,
}

pub struct SupplierDTO {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub address_id: uuid::Uuid,
    pub street_number: String,
    pub apartment: String,
    pub name: String,
    pub bank: String,
    pub bank_account_number: String,
    pub inn: String,
}
