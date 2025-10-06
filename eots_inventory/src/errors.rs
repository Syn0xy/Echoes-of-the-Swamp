#[derive(Debug)]
pub enum InventoryError {
    ItemNotFound,
    InvalidAmount(u32),
    NotEnoughQuantity { request: u32, available: u32 },
    QuantityOverflow,
}
