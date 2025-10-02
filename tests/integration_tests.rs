#[tokio::test]
async fn test_order_placement() {
    // Example test
    let order = crate::domain::order::Order {
        id: 1,
        market: "Bonk".to_string(),
        price: 9.5,
        quantity: 1.0,
        side: crate::domain::order::OrderSide::Buy,
    };
    let result = crate::application::trader_service::place_order(order).await;
    assert!(result.is_ok());
}
