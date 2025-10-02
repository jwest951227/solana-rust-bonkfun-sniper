// Represents an order in the trading system
#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub market: String,
    pub price: f64,
    pub quantity: f64,
    pub side: OrderSide,
}

#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}
