use super::find_waypoint_type_in_system;
use super::MARKET_TRAIT;
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::TradeGood;
use crate::spacetraders_api::{System, Waypoint};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub fn find_trade_good<'a>(trade_goods: &'a [TradeGood], to_find: &str) -> Option<&'a TradeGood> {
    trade_goods
        .iter()
        .find(|&trade_good| trade_good.symbol == to_find)
}

pub async fn find_trade_good_in_system(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    trade_good: &str,
    system: &System,
) -> Result<Option<Waypoint>, Box<dyn std::error::Error + Send + Sync>> {
    let market_waypoint =
        find_waypoint_type_in_system(client, sender, token, MARKET_TRAIT, system).await?; // CHECK this will only check the first market waypoint

    match market_waypoint {
        Some(market_waypoint) => {
            let marketplace_response = queries::market(
                client,
                sender,
                token,
                &market_waypoint.system(),
                &market_waypoint,
            )
            .await?;

            Ok(
                find_trade_good(&marketplace_response.trade_goods, trade_good)
                    .map(|_| market_waypoint),
            )
        }
        None => Ok(None),
    }
}
