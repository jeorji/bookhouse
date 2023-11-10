pub struct User {
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        User {
            email,
            password_hash,
        }
    }
}
