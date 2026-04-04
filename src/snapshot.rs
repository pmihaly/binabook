use serde::{Deserialize, Serialize};

use crate::types::PriceLevel;

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}
