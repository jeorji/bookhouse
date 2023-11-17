pub struct User {
    pub email: String,
    pub password_hash: String,
    pub group_id: i32,
}

impl User {
    pub fn new(email: String, password_hash: String, group_id: i32) -> Self {
        User {
            email,
            password_hash,
            group_id,
        }
    }
}
