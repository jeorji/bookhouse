use auth::ports::{
    repositories::permission::PermissionRepository,
    repositories::permission::{CreatePermissionDTO, FindPermissionDTO, PermissionDTO},
};
use super::{SQLXError, Error};

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

    async fn create(&self, dto: CreatePermissionDTO) -> Result<PermissionDTO, Error> {
        let query_result = sqlx::query!(
            r#"INSERT INTO "permission" VALUES ($1, $2) RETURNING *"#,
            dto.user_id,
            dto.group,
        )
        .fetch_one(&self.conncection_pool)
        .await
        .map_err(SQLXError)?;

        Ok(PermissionDTO {
            // add custom Error and remove unwrap
            user_id: query_result.user_id.unwrap(),
            group: query_result.group,
        })
    }
}
