pub struct Warehouse {
    pub name: String,
    pub location: (f64, f64),
    pub capacity: u32,
    pub inventory: u32,
}

impl Warehouse {
    pub fn new(name: String, location: (f64, f64), capacity: u32, inventory: u32) -> Self {
        Warehouse {
            name,
            location,
            capacity,
            inventory,
        }
    }
}
