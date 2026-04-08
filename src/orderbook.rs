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

const TICKS: usize = 100_000;
const BITS: usize = (TICKS / 64) + 1;

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
    fn set_bit(bitset: &mut [u64; BITS], price: Price) {
        bitset[(price.0 >> 6) as usize] |= 1 << (price.0 & 63);
    }

    fn clear_bit(bitset: &mut [u64; BITS], price: Price) {
        bitset[(price.0 >> 6) as usize] &= !(1 << (price.0 & 63));
    }

    pub fn apply_depth_update(&mut self, depth_update: &DepthUpdate) {
        if depth_update.final_update_id <= self.update_id {
            return;
        }

        self.update_id = depth_update.final_update_id;

        for bid in &depth_update.bids {
            self.bids[bid.price.0 as usize] = bid.quantity;

            match bid.quantity.0.cmp(&0) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    Self::clear_bit(&mut self.bid_bitset, bid.price)
                }
                std::cmp::Ordering::Greater => Self::set_bit(&mut self.bid_bitset, bid.price),
            }
        }

        for ask in &depth_update.asks {
            self.asks[ask.price.0 as usize] = ask.quantity;

            match ask.quantity.0.cmp(&0) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    Self::clear_bit(&mut self.ask_bitset, ask.price)
                }
                std::cmp::Ordering::Greater => Self::set_bit(&mut self.ask_bitset, ask.price),
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
