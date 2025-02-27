mod agent;
mod market;

fn main() {
    let mut market = market::market::Market::new(|volume| 0.01 * volume);
    market.agents.populate_market(50, 100);

    for _ in 0..5000 {
        market.tick();
        println!("{:?}", market.history.last().unwrap());
        market.agents.learn(&market.history);
    }
}
