use rust_decimal::Decimal;
use serde::{self, Deserialize, Serialize};
use std::fmt::Debug;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultData<T> {
    pub success: bool,
    pub(crate) result: T,
    pub has_more_data: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub success: bool,
    pub(crate) result: AccountInfo,
}


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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub cost: Decimal,
    pub entry_price: Decimal,
    pub future: String,
    pub initial_margin_requirement: Decimal,
    pub long_order_size: Decimal,
    pub maintenance_margin_requirement: Decimal,
    pub net_size: Decimal,
    pub open_size: Decimal,
    pub realized_pnl: Decimal,
    pub short_order_size: Decimal,
    pub side: String,
    pub size: Decimal,
    pub unrealized_pnl: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub account_identifier: i64,
    pub username: String,
    pub collateral: Decimal,
    pub free_collateral: Decimal,
    pub total_account_value: Decimal,
    pub total_position_size: Decimal,
    pub initial_margin_requirement: Decimal,
    pub maintenance_margin_requirement: Decimal,
    pub margin_fraction: Option<Decimal>,
    pub open_margin_fraction: Option<Decimal>,
    pub liquidating: bool,
    pub backstop_provider: bool,
    pub positions: Vec<Position>,
    pub taker_fee: Decimal,
    pub maker_fee: Decimal,
    pub leverage: Decimal,
    pub position_limit: Option<Decimal>,
    pub position_limit_used: Option<Decimal>,
    pub use_ftt_collateral: bool,
    pub charge_interest_on_negative_usd: bool,
    pub spot_margin_enabled: bool,
    pub spot_lending_enabled: bool,
}