mod matching_engine;

use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};

fn main() {
    let mut orderbook = Orderbook::new();

    let bid = Order::new(BidOrAsk::Bid, 5.5);
    let ask = Order::new(BidOrAsk::Ask, 2.45);
    let ask2 = Order::new(BidOrAsk::Ask, 2.45);

    let mut engine = MatchingEngine::new();
    let trading_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    
    engine.add_new_market(trading_pair.clone());
    let result = engine.place_limit_order(trading_pair.clone(), 5.5, bid);

    println!("{:?}", engine);
}
