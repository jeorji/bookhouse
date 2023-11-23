pub mod user;
pub mod session;
pub mod permission;

use auth::ports::Error;
pub struct SQLXError(sqlx::Error);
impl From<SQLXError> for Error {
    fn from(err: SQLXError) -> Self {
        Error::InfrastructureError(Box::new(err.0))
    }
}
