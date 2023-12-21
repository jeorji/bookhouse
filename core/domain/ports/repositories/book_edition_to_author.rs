use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait BookEditionToAuthorRepository:
    FindRepo<FindBookEditionIdDTO, BookEditionToAuthorDTO>
    + CreateRepo<CreateBookEditionToAuthorDTO, BookEditionToAuthorDTO>
{
}

pub struct FindBookEditionIdDTO {
    pub book_edition_id: uuid::Uuid,
}

pub struct CreateBookEditionToAuthorDTO {
    pub book_edition_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
}

pub struct BookEditionToAuthorDTO {
    pub book_edition_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
}
