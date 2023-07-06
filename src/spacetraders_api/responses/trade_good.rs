use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeGood {
    pub symbol: String,
    #[serde(rename = "tradeVolume")]
    pub trade_volume: u128,
    pub supply: String,
    #[serde(rename = "purchasePrice")]
    pub purchase_price: i128,
    #[serde(rename = "sellPrice")]
    pub sell_price: i128,
}
