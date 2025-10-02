// Represents a trade execution
#[derive(Debug, Clone)]
pub struct Trade {
    pub order_id: u64,
    pub market: String,
    pub executed_price: f64,
    pub quantity: f64,
    pub timestamp: u64,
}
