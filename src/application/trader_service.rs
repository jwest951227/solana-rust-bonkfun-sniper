use crate::domain::{order::Order, trade::Trade};
use crate::infrastructure::solana_client;

pub struct TraderService;

impl TraderService {
    pub async fn place_order(order: Order) -> Result<Trade, String> {
        // Placeholder for placing order via Solana client
        println!("Placing order: {:?}", order);
        // Call infrastructure to send transaction
        let trade = solana_client::execute_order(&order).await?;
        Ok(trade)
    }
}
