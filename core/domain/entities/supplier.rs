pub struct Supplier {
    pub user_id: uuid::Uuid,
    pub address_id: uuid::Uuid,
    pub street_number: String,
    pub apartment: String,
    pub name: String,
    pub bank: String,
    pub bank_account_number: String,
    pub inn: String,
}

impl Supplier {
    pub fn new(
        user_id: uuid::Uuid,
        address_id: uuid::Uuid,
        street_number: String,
        apartment: String,
        name: String,
        bank: String,
        bank_account_number: String,
        inn: String,
    ) -> Self {
        Supplier {
            user_id,
            address_id,
            street_number,
            apartment,
            name,
            bank,
            bank_account_number,
            inn,
        }
    }
}
