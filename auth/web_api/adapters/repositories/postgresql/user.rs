use auth::ports::{
    repositories::user::UserRepository,
    repositories::user::{CreateUserDTO, FindUserDTO, UserDTO},
};
use super::{SQLXError, Error};

pub struct PostgresUserRepository {
    pub conncection_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, dto: CreateUserDTO) -> Result<UserDTO, Error> {
        let query_result = sqlx::query!(
            r#"INSERT INTO "user" VALUES (uuid_generate_v4(), $1, $2, $3) RETURNING *"#,
            dto.email,
            dto.password_hash,
            dto.salt
        )
        .fetch_one(&self.conncection_pool)
        .await
        .map_err(SQLXError)?;

        Ok(UserDTO {
            id: query_result.id,
            email: query_result.email,
            password_hash: query_result.password_hash,
            salt: query_result.salt,
        })
    }

    async fn find(&self, dto: FindUserDTO) -> Result<Option<UserDTO>, Error> {
        let query_result = sqlx::query!(r#"SELECT * FROM "user" WHERE email = $1"#, dto.email)
            .fetch_optional(&self.conncection_pool)
            .await
            .map_err(SQLXError)?;

        Ok(query_result.map(|record| UserDTO {
            id: record.id,
            email: record.email,
            password_hash: record.password_hash,
            salt: record.salt,
        }))
    }
}
