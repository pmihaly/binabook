use serde::{Deserialize, Serialize};

use crate::types::{PriceLevel, UpdateID};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    #[serde(rename(deserialize = "lastUpdateId"))]
    pub last_update_id: UpdateID,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}
