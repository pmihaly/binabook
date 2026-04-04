use crate::types::{PriceLevel, Symbol};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthUpdate {
    #[serde(rename(deserialize = "s"))]
    pub symbol: Symbol,

    #[serde(rename(deserialize = "b"))]
    pub bids: Vec<PriceLevel>,

    #[serde(rename(deserialize = "a"))]
    pub asks: Vec<PriceLevel>,
}
