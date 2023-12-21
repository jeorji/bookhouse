pub struct Author {
    pub first_name: String,
    pub middle_name: String,
    pub second_name: String,
    pub description: String,
    pub birth_date: u32,
    pub death_date: u32,
}

impl Author {
    pub fn new(
        first_name: String,
        middle_name: String,
        second_name: String,
        description: String,
        birth_date: u32,
        death_date: u32,
    ) -> Self {
        Author {
            first_name,
            middle_name,
            second_name,
            description,
            birth_date,
            death_date,
        }
    }
}
