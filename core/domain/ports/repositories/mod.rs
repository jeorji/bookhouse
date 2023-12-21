pub mod author;
pub mod book_edition;
pub mod book_edition_to_author;
pub mod genre;
pub mod inventory;
pub mod supplier;
pub mod supply;
pub mod warehouse;

#[async_trait::async_trait]
pub trait FindRepo<T, R> {
    async fn find(&self, dto: T) -> Result<Option<R>, super::Error>;
}

#[async_trait::async_trait]
pub trait CreateRepo<T, R> {
    async fn create(&self, dto: T) -> Result<R, super::Error>;
}

#[async_trait::async_trait]
pub trait UpdateRepo<T, R> {
    async fn update(&self, dto: T) -> Result<Option<R>, super::Error>;
}

#[async_trait::async_trait]
pub trait DeleteRepo<T> {
    async fn delete(&self, dto: T) -> Result<bool, super::Error>;
}
