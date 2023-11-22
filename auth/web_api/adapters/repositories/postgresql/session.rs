use auth::ports::{
    repositories::session::SessionRepository,
    repositories::session::{CreateSessionDTO, SessionDTO},
    Error,
};

pub struct SQLXError(sqlx::Error);
impl From<SQLXError> for Error {
    fn from(err: SQLXError) -> Self {
        Error::InfrastructureError(Box::new(err.0))
    }
}
pub struct PostgresSessionRepository {
    pub conncection_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn create(&self, dto: CreateSessionDTO) -> Result<SessionDTO, Error> {
        let query_result = sqlx::query!(
            r#"INSERT INTO "session" VALUES (uuid_generate_v4(), $1, $2, to_timestamp($3)) RETURNING id"#,
            dto.user_id,
            dto.refresh_token,
            dto.refresh_token_exp as f64,
        )
        .fetch_one(&self.conncection_pool)
        .await
        .map_err(SQLXError)?;

        Ok(SessionDTO {
            id: query_result.id,
            user_id: dto.user_id,
            refresh_token: dto.refresh_token,
            refresh_token_exp: dto.refresh_token_exp,
        })
    }
}
