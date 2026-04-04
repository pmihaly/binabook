use crate::types::{PriceLevel, Symbol};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthUpdate {
    #[serde(rename(deserialize = "s"))]
    symbol: Symbol,

    #[serde(rename(deserialize = "b"))]
    bids: Vec<PriceLevel>,

    #[serde(rename(deserialize = "a"))]
    asks: Vec<PriceLevel>,
}
