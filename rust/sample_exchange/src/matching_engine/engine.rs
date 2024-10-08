use std::collections::HashMap;

use super::orderbook::{Order, Orderbook};

// i.e.  BTCUSD
//   BTC => BASE
//   USD => QUOTE

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote
        }
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new()
        }
    }

    pub fn add_new_market(&mut self, trading_pair: &TradingPair) {
        match self.orderbooks.get_key_value(&trading_pair) {
            Some(_) => {}
            None => {
                self.orderbooks.insert(trading_pair.clone(), Orderbook::new());
            }
        }
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: f64, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                Ok(())
            },
            None => { 
                Err(format!("The orderbook for: ({}) does not exist", pair.to_string()))
            }
        }
    }
}