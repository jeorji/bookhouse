pub struct SupplyContent {
    pub supply_id: uuid::Uuid,
    pub book_id: uuid::Uuid,
    pub quanity: u32,
    /// in penny
    pub price: u32,
}

impl SupplyContent {
    pub fn new(supply_id: uuid::Uuid, book_id: uuid::Uuid, quanity: u32, price: u32) -> Self {
        SupplyContent {
            supply_id,
            book_id,
            quanity,
            price,
        }
    }
}
