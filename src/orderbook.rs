use std::{cmp::Reverse, collections::BTreeMap};

use crate::{
    snapshot::Snapshot,
    types::{Price, Quantity},
};

#[derive(Debug, Default)]
pub struct Orderbook {
    pub bids: BTreeMap<Price, Quantity>,
    pub asks: BTreeMap<Reverse<Price>, Quantity>,
}

impl From<Snapshot> for Orderbook {
    fn from(value: Snapshot) -> Self {
        let bids = BTreeMap::from_iter(
            value
                .bids
                .iter()
                .map(|price_level| (price_level.price, price_level.quantity)),
        );
        let asks = BTreeMap::from_iter(
            value
                .asks
                .iter()
                .map(|price_level| (Reverse(price_level.price), price_level.quantity)),
        );

        Self { bids, asks }
    }
}
