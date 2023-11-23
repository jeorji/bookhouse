use auth::ports::{
    repositories::session::SessionRepository,
    repositories::session::{CreateSessionDTO, FindSessionDTO, SessionDTO, UpdateSessionDTO},
};
use super::{SQLXError, Error};

pub struct PostgresSessionRepository {
    pub conncection_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn create(&self, dto: CreateSessionDTO) -> Result<SessionDTO, Error> {
        let query_result = sqlx::query!(
            r#"INSERT INTO "session"
            VALUES (uuid_generate_v4(), $1, $2, to_timestamp($3)) RETURNING id"#,
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

    async fn update(&self, dto: UpdateSessionDTO) -> Result<Option<SessionDTO>, Error> {
        let query_result = sqlx::query!(
            r#"UPDATE "session"
            SET refresh_token = $1,
                refresh_token_exp = to_timestamp($2)
            WHERE refresh_token = $3 RETURNING id, user_id"#,
            dto.new_refresh_token,
            dto.new_refresh_token_exp as f64,
            dto.refresh_token,
        )
        .fetch_optional(&self.conncection_pool)
        .await
        .map_err(SQLXError)?;

        Ok(query_result.map(|record| SessionDTO {
            id: record.id,
            user_id: record.user_id,
            refresh_token: dto.new_refresh_token,
            refresh_token_exp: dto.new_refresh_token_exp,
        }))
    }

    async fn find(&self, dto: FindSessionDTO) -> Result<Option<SessionDTO>, Error> {
        let query_result = sqlx::query!(
            r#"SELECT id,
                   user_id,
                   refresh_token,
                   EXTRACT(EPOCH
                           FROM refresh_token_exp)::BIGINT AS refresh_token_exp
            FROM "session"
            WHERE refresh_token = $1"#,
            dto.refresh_token,
        )
        .fetch_optional(&self.conncection_pool)
        .await
        .map_err(SQLXError)?;

        Ok(query_result.map(|record| SessionDTO {
            id: record.id,
            user_id: record.user_id,
            refresh_token: record.refresh_token,
            refresh_token_exp: record.refresh_token_exp.unwrap_or(0i64) as u64,
        }))
    }
}
