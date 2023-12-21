pub struct Inventory {
    pub book_id: uuid::Uuid,
    pub warehouse_id: uuid::Uuid,
    pub stock: u32,
}

impl Inventory {
    pub fn new(book_id: uuid::Uuid, warehouse_id: uuid::Uuid, stock: u32) -> Self {
        Inventory {
            book_id,
            warehouse_id,
            stock,
        }
    }
}
