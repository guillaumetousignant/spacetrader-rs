use super::Good;
use super::MarketTransaction;
use super::TradeGood;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub symbol: Waypoint,
    pub imports: Vec<Good>,
    pub exports: Vec<Good>,
    pub exchange: Vec<Good>,
    pub transactions: Vec<MarketTransaction>,
    #[serde(rename = "tradeGoods")]
    pub trade_goods: Vec<TradeGood>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketResponse {
    pub data: Market,
}
