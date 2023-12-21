pub struct Supply {
    pub supplier_id: uuid::Uuid,
    pub status: SupplyStatus,
    pub warehouse_id: uuid::Uuid,
    pub date: u32,
}

pub enum SupplyStatus {
    Created,
    Accepted,
    Rejected,
    Completed,
}

impl Supply {
    pub fn new(
        supplier_id: uuid::Uuid,
        warehouse_id: uuid::Uuid,
        status: SupplyStatus,
        date: u32,
    ) -> Self {
        Supply {
            supplier_id,
            status,
            warehouse_id,
            date,
        }
    }
}
