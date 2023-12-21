use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait GenreRepository:
    FindRepo<FindByIdDTO, GenreDTO> + CreateRepo<CreateGenreDTO, GenreDTO>
{
}

pub struct FindByIdDTO{
    pub id: uuid::Uuid,
}

pub struct CreateGenreDTO {
    pub name: String,
    pub description: String,
}

pub struct GenreDTO {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
}
