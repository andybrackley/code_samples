use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk { Bid, Ask }

#[derive(Debug)]
struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl Orderbook {
    fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new()
        }
    }

    fn add_order(&mut self,  price_value: f64, order: Order) {
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
struct Price {
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
struct Limit {
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

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
}

fn main() {
    let mut orderbook = Orderbook::new();

    let bid = Order::new(BidOrAsk::Bid, 5.5);
    let ask = Order::new(BidOrAsk::Ask, 2.45);
    let ask2 = Order::new(BidOrAsk::Ask, 2.45);


    orderbook.add_order(5.5, bid);
    orderbook.add_order(2.5, ask);
    orderbook.add_order(2.5, ask2);


    println!("{:?}", orderbook);
}
