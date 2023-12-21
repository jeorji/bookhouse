pub struct BookEdition {
    pub title: String,
    pub genre_id: uuid::Uuid,
    pub description: String,
    pub pages: u32,
    pub release: u32,
    /// in penny
    pub price: u32,
}

impl BookEdition {
    pub fn new(
        title: String,
        genre_id: uuid::Uuid,
        description: String,
        pages: u32,
        release: u32,
        price: u32,
    ) -> Self {
        BookEdition {
            title,
            genre_id,
            description,
            pages,
            release,
            price,
        }
    }
}
