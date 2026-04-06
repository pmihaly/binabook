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

#[derive(Debug, Default)]
pub struct Orderbook {
    prev_update_id: UpdateID,
    bids: BTreeMap<Reverse<Price>, Quantity>,
    asks: BTreeMap<Price, Quantity>,
}

impl Orderbook {
    pub fn apply_depth_update(&mut self, depth_update: DepthUpdate) {
        let is_stale_update = depth_update.final_update_id <= self.prev_update_id;
        if is_stale_update {
            self.prev_update_id = depth_update.final_update_id;
            return;
        }

        let has_missed_an_update = depth_update.prev_final_update_id != self.prev_update_id;

        if has_missed_an_update {
            panic!("missed a depth update, refetching snapshot is not implemented")
        }

        self.prev_update_id = depth_update.final_update_id;

        depth_update.bids.iter().for_each(|bid| {
            if bid.quantity == Quantity::default() {
                self.bids.remove_entry(&Reverse(bid.price));
                return;
            }
            self.bids
                .entry(Reverse(bid.price))
                .insert_entry(bid.quantity);
        });

        depth_update.asks.iter().for_each(|ask| {
            if ask.quantity == Quantity::default() {
                self.asks.remove_entry(&ask.price);
                return;
            }
            self.asks.entry(ask.price).insert_entry(ask.quantity);
        });
    }

    pub fn display_top_levels(&self, top_levels: usize) -> String {
        let mut output: String = "bids:\n~~~\n".into();

        for best_bid in self.bids.iter().take(top_levels) {
            output += &format!("{}: {}\n", best_bid.0.0, best_bid.1);
        }

        output += "\nasks:\n~~~\n";
        for best_ask in self.asks.iter().take(top_levels) {
            output += &format!("{}: {}\n", best_ask.0, best_ask.1);
        }
        output += "\n\n";

        output
    }
}

impl From<Snapshot> for Orderbook {
    fn from(value: Snapshot) -> Self {
        let bids = BTreeMap::from_iter(
            value
                .bids
                .iter()
                .map(|price_level| (Reverse(price_level.price), price_level.quantity)),
        );
        let asks = BTreeMap::from_iter(
            value
                .asks
                .iter()
                .map(|price_level| (price_level.price, price_level.quantity)),
        );

        Self {
            bids,
            asks,
            prev_update_id: value.last_update_id,
        }
    }
}
