use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize};

fn from_str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f32>().map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Symbol(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord)]
pub struct Price(pub u32);

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Quantity(pub u32);

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct UpdateID(pub i64);

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceLevel {
    pub price: Price,
    pub quantity: Quantity,
}
