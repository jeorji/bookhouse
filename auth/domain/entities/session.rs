pub struct Session {
    pub user_id: uuid::Uuid,
    pub refresh_token: String,
    pub refresh_token_exp: u64,
}

impl Session {
    pub fn new(
        user_id: uuid::Uuid,
        refresh_token: String,
        refresh_token_exp: u64,
    ) -> Self {
        Session {
            user_id,
            refresh_token,
            refresh_token_exp,
        }
    }
}
