pub struct Genre {
    pub name: String,
    pub description: String,
}

impl Genre {
    pub fn new(name: String, description: String) -> Self {
        Genre { name, description }
    }
}
