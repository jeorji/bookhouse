pub struct BookEditionToAuthor {
    pub book_edition: uuid::Uuid,
    pub author_id: uuid::Uuid,
}

impl BookEditionToAuthor {
    pub fn new(book_edition: uuid::Uuid, author_id: uuid::Uuid) -> Self {
        BookEditionToAuthor {
            book_edition,
            author_id,
        }
    }
}
