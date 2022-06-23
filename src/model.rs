use rust_decimal::Decimal;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    pub start_time: String,
    pub time: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}
