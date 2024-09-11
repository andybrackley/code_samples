use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum BidOrAsk { Bid, Ask }

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new()
        }
    }

    pub fn get_limits(&mut self, side: BidOrAsk) -> Vec<&mut Limit> {
        let limits = if side == BidOrAsk::Bid { &mut self.bids } else { &mut self.asks };
        let ordered = limits.values_mut().collect::<Vec<&mut Limit>>();
        ordered
    }


    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let opposite_side = if market_order.bid_or_ask == BidOrAsk::Bid { BidOrAsk::Ask } else { BidOrAsk::Bid };
        let limits = self.get_limits(opposite_side);

        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn add_order(&mut self,  price_value: f64, order: Order) {
        let price = Price::new(price_value);
        let mut lookup = 
            match order.bid_or_ask {
                BidOrAsk::Bid => &mut self.bids,
                BidOrAsk::Ask => &mut self.asks
            };

        let limit_opt = lookup.get_mut(&price) ;
        match limit_opt {
            Some(limit) => limit.add_order(order),
            None => {
                let mut limit = Limit::new(price); 
                limit.add_order(order);
                lookup.insert(price, limit);
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    scalar: u64,
    integral: u64,
    fractional: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = (( price % 1.0 ) * scalar as f64) as u64;
        Price { 
            scalar, 
            integral, 
            fractional
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price, 
            orders: Vec::new()
        }
    }

    fn total_volume(&self) -> f64 {
        let volume: f64 = self.orders
                .iter()
                .map(|order| order.size)
                .reduce(|a,b| a + b)
                .unwrap_or(0.0);

        volume
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_total_volume_when_empty_should_equal_0() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        assert_eq!(0.0, limit.total_volume());
    }

    #[test]
    fn limit_total_volume_should_equal_total_of_orders() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let size = 5.0;
        [0; 6].iter()
            .for_each(|_| { 
                limit.add_order(Order::new(BidOrAsk::Bid, size)); 
            });

        assert_eq!(6.0 * size, limit.total_volume());
    }


    #[test]
    fn limit_order_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);

        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(true, market_sell_order.is_filled());
        // assert_eq!(1.0, buy_limit_order.size);
    }
}