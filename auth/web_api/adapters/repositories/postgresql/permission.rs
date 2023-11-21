use auth::ports::{
    repositories::permission::PermissionRepository,
    repositories::permission::{FindPermissionDTO, PermissionDTO},
    Error,
};

pub struct SQLXError(sqlx::Error);
impl From<SQLXError> for Error {
    fn from(err: SQLXError) -> Self {
        Error::InfrastructureError(Box::new(err.0))
    }
}
pub struct PostgresPermissonRepository {
    pub conncection_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl PermissionRepository for PostgresPermissonRepository {
    async fn find(&self, dto: FindPermissionDTO) -> Result<Option<PermissionDTO>, Error> {
        let query_result = sqlx::query!(
            r#"SELECT "group" FROM "permission" WHERE user_id = $1"#,
            dto.user_id,
        )
        .fetch_optional(&self.conncection_pool)
        .await
        .map_err(SQLXError)?;

        Ok(query_result.map(|record| PermissionDTO {
            user_id: dto.user_id,
            group: record.group,
        }))
    }
}
