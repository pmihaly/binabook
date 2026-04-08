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

const MID_PRICE: usize = 60_000;
const TICKS_PER_SIDE: usize = 20_000;
const MIN_PRICE: usize = MID_PRICE - TICKS_PER_SIDE;
const TICKS: usize = TICKS_PER_SIDE * 2;
const BITS: usize = (TICKS + 63) / 64;

#[derive(Debug)]
pub struct Orderbook {
    update_id: UpdateID,
    bids: [Quantity; TICKS],
    asks: [Quantity; TICKS],
    bid_bitset: [u64; BITS],
    ask_bitset: [u64; BITS],
}

impl Default for Orderbook {
    fn default() -> Self {
        Orderbook {
            update_id: UpdateID::default(),
            bids: [Quantity(0); TICKS],
            asks: [Quantity(0); TICKS],
            bid_bitset: [0; BITS],
            ask_bitset: [0; BITS],
        }
    }
}

impl Orderbook {
    fn set_bit(bitset: &mut [u64; BITS], index: usize) {
        bitset[index >> 6] |= 1 << (index & 63);
    }

    fn clear_bit(bitset: &mut [u64; BITS], index: usize) {
        bitset[index >> 6] &= !(1 << (index & 63));
    }

    pub fn apply_depth_update(&mut self, depth_update: &DepthUpdate) {
        if depth_update.final_update_id <= self.update_id {
            return;
        }

        self.update_id = depth_update.final_update_id;

        for bid in &depth_update.bids {
            let index = bid.price.0 as usize - MIN_PRICE;
            let prev = self.bids[index];

            self.bids[index] = bid.quantity;

            match (prev.0, bid.quantity.0) {
                (0, 0) => continue,
                (_, 0) => Self::clear_bit(&mut self.bid_bitset, index),
                (_, _) => Self::set_bit(&mut self.bid_bitset, index),
            }
        }

        for ask in &depth_update.asks {
            let index = ask.price.0 as usize - MIN_PRICE;
            let prev = self.asks[index];
            self.asks[index] = ask.quantity;

            match (prev.0, ask.quantity.0) {
                (0, 0) => continue,
                (_, 0) => Self::clear_bit(&mut self.ask_bitset, index),
                (_, _) => Self::set_bit(&mut self.ask_bitset, index),
            }
        }
    }
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
