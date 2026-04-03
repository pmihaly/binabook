use serde::{Deserialize, Deserializer, Serialize};

fn from_str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f32>().map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
struct Symbol(String);

#[derive(Serialize, Deserialize, Debug)]
struct Price(#[serde(deserialize_with = "from_str_to_f32")] f32);

#[derive(Serialize, Deserialize, Debug)]
struct Quantity(#[serde(deserialize_with = "from_str_to_f32")] f32);

#[derive(Serialize, Deserialize, Debug)]
pub struct SideUpdate {
    price_level: Price,
    quantity: Quantity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthUpdate {
    #[serde(rename(deserialize = "s"))]
    symbol: Symbol,

    #[serde(rename(deserialize = "b"))]
    bids: Vec<SideUpdate>,

    #[serde(rename(deserialize = "a"))]
    asks: Vec<SideUpdate>,
}
