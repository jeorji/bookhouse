use super::CreateRepo;
use super::FindRepo;

#[async_trait::async_trait]
pub trait BookEditionToAuthorRepository:
    FindRepo<FindByBookEditionIdDTO, BookEditionToAuthorDTO>
    + FindRepo<FindByAuthorIdDTO, BookEditionToAuthorDTO>
    + CreateRepo<CreateBookEditionToAuthorDTO, BookEditionToAuthorDTO>
{
}

pub struct FindByBookEditionIdDTO {
    pub book_edition_id: uuid::Uuid,
}

pub struct FindByAuthorIdDTO {
    pub author_id: uuid::Uuid,
}

pub struct CreateBookEditionToAuthorDTO {
    pub book_edition_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
}

pub struct BookEditionToAuthorDTO {
    pub book_edition_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
}
