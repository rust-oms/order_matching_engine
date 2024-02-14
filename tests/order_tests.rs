use order_matching_engine::models::{order_type::OrderType, side::Side};
use order_matching_engine::order::Order;
use rust_decimal::Decimal;

#[test]
fn create_order_test1() {
    let order = Order::new(
        OrderType::Limit,
        Decimal::new(10, 2),
        Decimal::new(100, 2),
        "Symbol1".to_string(),
        Side::Bid,
    );
    assert_eq!(order.order_type, OrderType::Limit);
    assert_eq!(order.volume, Decimal::new(10, 2));
    assert_eq!(order.price, Decimal::new(100, 2));
    assert_eq!(order.symbol, "Symbol1");
    assert_eq!(order.side, Side::Bid);
}

#[test]
fn create_order_test2() {
    let order = Order::new(
        OrderType::Market,
        Decimal::new(50, 2),
        Decimal::new(70, 2),
        "Symbol2".to_string(),
        Side::Ask,
    );
    assert_eq!(order.order_type, OrderType::Market);
    assert_eq!(order.volume, Decimal::new(50, 2));
    assert_eq!(order.price, Decimal::new(70, 2));
    assert_eq!(order.symbol, "Symbol2");
    assert_eq!(order.side, Side::Ask);
}
