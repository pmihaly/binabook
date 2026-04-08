use std::time::Instant;

use rand::RngExt;

use crate::depth_update::DepthUpdate;
use crate::orderbook::Orderbook;
use crate::types::{Price, PriceLevel, Quantity, Symbol, UpdateID};

mod depth_update;
mod orderbook;
mod snapshot;
mod types;

const N_UPDATES: usize = 1_000_000;
const RUNS: usize = 100;

fn random_depth_update(prev_final_update_id: UpdateID, symbol: &Symbol) -> DepthUpdate {
    let mut rng = rand::rng();

    let prev = prev_final_update_id.0;
    let first = prev + 1;
    let final_id = first + rng.random_range(0..3);

    let mid_price: u32 = 60_000;

    let min_price = mid_price - 20_000;
    let max_price = mid_price + 20_000;

    let bid_levels = rng.random_range(1..5);
    let ask_levels = rng.random_range(1..5);

    let mut bids = Vec::with_capacity(bid_levels);
    let mut asks = Vec::with_capacity(ask_levels);

    for i in 0..bid_levels {
        // Price offset from mid price
        let price = (mid_price - (i as u32 + 1) * rng.random_range(0..5)).max(min_price); // ensure >= min_price
        let qty = if rng.random_bool(0.1) {
            0
        } else {
            rng.random_range(01..5000000)
        };

        bids.push(PriceLevel {
            price: Price(price),
            quantity: Quantity(qty),
        });
    }

    for i in 0..ask_levels {
        let price = (mid_price + (i as u32 + 1) * rng.random_range(0..5)).min(max_price); // ensure <= max_price
        let qty = if rng.random_bool(0.1) {
            0
        } else {
            rng.random_range(01..5000000)
        };

        asks.push(PriceLevel {
            price: Price(price),
            quantity: Quantity(qty),
        });
    }

    DepthUpdate {
        first_update_id: UpdateID(first),
        final_update_id: UpdateID(final_id),
        prev_final_update_id,
        symbol: Symbol(symbol.0.clone()),
        bids,
        asks,
    }
}
fn main() {
    println!("generating {N_UPDATES} random updates...");

    let symbol = Symbol("BTCUSDT".to_string());
    let mut last_id = UpdateID(1);

    let mut updates = Vec::with_capacity(N_UPDATES);

    for _ in 0..N_UPDATES {
        let update = random_depth_update(last_id, &symbol);
        last_id = update.final_update_id;
        updates.push(update);
    }

    println!("updates generated\n");

    let mut results = Vec::with_capacity(RUNS);

    for run in 0..RUNS {
        let mut orderbook = Orderbook::default();

        let start = Instant::now();

        for update in &updates {
            orderbook.apply_depth_update(update);
        }

        let elapsed = start.elapsed().as_secs_f64();
        let throughput = N_UPDATES as f64 / elapsed;

        println!(
            "run {}: {:.3}s | {:.0} updates/sec",
            run + 1,
            elapsed,
            throughput
        );

        results.push(throughput);
    }

    let avg = results.iter().sum::<f64>() / RUNS as f64;

    println!("\naverage throughput: {:.0} updates/sec", avg);
}
