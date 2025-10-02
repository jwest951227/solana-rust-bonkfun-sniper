use crate::domain::market::Market;

pub struct StrategyService;

impl StrategyService {
    pub fn generate_signal(market: &Market) -> Option<Order> {
        // Placeholder logic: simple buy if price below a threshold
        if market.current_price < 10.0 {
            Some(Order {
                id: 0,
                market: market.name.clone(),
                price: market.current_price,
                quantity: 1.0,
                side: crate::domain::order::OrderSide::Buy,
            })
        } else {
            None
        }
    }
}
