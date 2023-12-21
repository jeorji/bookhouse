use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait AuthorRepository:
    FindRepo<FindByIdDTO, AuthorDTO> + CreateRepo<CreateAuthorDTO, AuthorDTO>
{
}

pub struct FindByIdDTO {
    pub id: uuid::Uuid,
}

pub struct CreateAuthorDTO {
    pub first_name: String,
    pub middle_name: String,
    pub second_name: String,
    pub description: String,
    pub birth_date: u32,
    pub death_date: u32,
}

pub struct AuthorDTO {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub middle_name: String,
    pub second_name: String,
    pub description: String,
    pub birth_date: u32,
    pub death_date: u32,
}
