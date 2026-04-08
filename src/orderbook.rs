use std::{cmp::Reverse, collections::BTreeMap};

use crate::{
    depth_update::DepthUpdate,
    snapshot::Snapshot,
    types::{Price, Quantity, UpdateID},
};

#[derive(Debug)]
pub enum OrderbookEvent {
    SnapshotUpdate(Snapshot),
    DepthUpdate(DepthUpdate),
}

type PriceTicks = u32;

fn price_to_index(price: PriceTicks) -> usize {
    price as usize
}

const TICKS: usize = 1_000_000;

#[derive(Debug)]
pub struct Orderbook {
    update_id: UpdateID,
    bids: [Quantity; TICKS],
    asks: [Quantity; TICKS],
}

impl Default for Orderbook {
    fn default() -> Self {
        Orderbook {
            update_id: UpdateID::default(),
            bids: [Quantity(0.0); TICKS],
            asks: [Quantity(0.0); TICKS],
        }
    }
}

impl Orderbook {
    pub fn apply_depth_update(&mut self, depth_update: &DepthUpdate) {
        if depth_update.final_update_id <= self.update_id {
            return;
        }

        self.update_id = depth_update.final_update_id;

        for bid in &depth_update.bids {
            let idx = bid.price.0 as usize;

            self.bids[idx] = bid.quantity;
        }

        for ask in &depth_update.asks {
            let idx = ask.price.0 as usize;

            self.asks[idx] = ask.quantity;
        }

        // // BIDS
        // for bid in &depth_update.bids {
        //     let idx = bid.price.0 as usize;
        //
        //     if idx >= self.bids.len() {
        //         self.bids.resize(idx + 1, zero);
        //     }
        //
        //     self.bids[idx] = bid.quantity;
        //
        //     if bid.quantity != zero {
        //         if idx > self.best_bid {
        //             self.best_bid = idx;
        //         }
        //     } else if idx == self.best_bid {
        //         while self.best_bid > 0 && self.bids[self.best_bid] == zero {
        //             self.best_bid -= 1;
        //         }
        //     }
        // }
        //
        // // ASKS
        // for ask in &depth_update.asks {
        //     let idx = ask.price.0 as usize;
        //
        //     if idx >= self.asks.len() {
        //         self.asks.resize(idx + 1, zero);
        //     }
        //
        //     self.asks[idx] = ask.quantity;
        //
        //     if ask.quantity != zero {
        //         if idx < self.best_ask || self.best_ask == 0 {
        //             self.best_ask = idx;
        //         }
        //     } else if idx == self.best_ask {
        //         while self.best_ask < self.asks.len() && self.asks[self.best_ask] == zero {
        //             self.best_ask += 1;
        //         }
        //     }
        // }
    }
    // pub fn display_top_levels(&self, top_levels: usize) -> String {
    //     let mut output: String = "bids:\n~~~\n".into();
    //
    //     for best_bid in self.bids.iter().take(top_levels) {
    //         output += &format!("{}: {}\n", best_bid.0.0, best_bid.1);
    //     }
    //
    //     output += "\nasks:\n~~~\n";
    //     for best_ask in self.asks.iter().take(top_levels) {
    //         output += &format!("{}: {}\n", best_ask.0, best_ask.1);
    //     }
    //     output += "\n\n";
    //
    //     output
    // }
}

// impl From<Snapshot> for Orderbook {
//     fn from(value: Snapshot) -> Self {
//         let bids = BTreeMap::from_iter(
//             value
//                 .bids
//                 .iter()
//                 .map(|price_level| (Reverse(price_level.price), price_level.quantity)),
//         );
//         let asks = BTreeMap::from_iter(
//             value
//                 .asks
//                 .iter()
//                 .map(|price_level| (price_level.price, price_level.quantity)),
//         );
//
//         Self {
//             bids,
//             asks,
//             update_id: value.update_id,
//         }
//     }
// }
