use crate::types::{PriceLevel, Symbol, UpdateID};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthUpdate {
    #[serde(rename(deserialize = "U"))]
    pub first_update_id: UpdateID,

    #[serde(rename(deserialize = "u"))]
    pub final_update_id: UpdateID,

    #[serde(rename(deserialize = "pu"))]
    pub prev_final_update_id: UpdateID,

    #[serde(rename(deserialize = "s"))]
    pub symbol: Symbol,

    #[serde(rename(deserialize = "b"))]
    pub bids: Vec<PriceLevel>,

    #[serde(rename(deserialize = "a"))]
    pub asks: Vec<PriceLevel>,
}
