// Represents market data
#[derive(Debug, Clone)]
pub struct Market {
    pub name: String,
    pub current_price: f64,
    pub volume: f64,
}
