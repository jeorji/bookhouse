pub struct Address {
    pub id: uuid::Uuid,
    pub street_name: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
}

impl Address {
    pub fn new(
        id: uuid::Uuid,
        street_name: String,
        city: String,
        region: String,
        postal_code: String,
        country: String,
    ) -> Self {
        Address {
            id,
            street_name,
            city,
            region,
            postal_code,
            country,
        }
    }
}
