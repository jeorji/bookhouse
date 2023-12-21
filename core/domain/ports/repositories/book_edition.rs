use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait BookEditionRepository:
    FindRepo<FindByIdDTO, BookEditionDTO> + CreateRepo<CreateBookEditionDTO, BookEditionDTO>
{
}

pub struct FindByIdDTO {
    pub id: uuid::Uuid,
}

pub struct CreateBookEditionDTO {
    pub title: String,
    pub genre_id: String,
    pub description: String,
    pub pages: u32,
    pub release: u32,
    pub price: u32,
}

pub struct BookEditionDTO {
    pub id: uuid::Uuid,
    pub title: String,
    pub genre_id: String,
    pub description: String,
    pub pages: u32,
    pub release: u32,
    pub price: u32,
}
