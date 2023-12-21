pub mod repositories;
pub mod services;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InfrastructureError(Box<dyn std::error::Error>)
}
