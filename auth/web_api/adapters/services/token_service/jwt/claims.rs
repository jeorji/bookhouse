#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub user_id: uuid::Uuid,
    pub permission_group: i32,
    pub exp: usize,
}
