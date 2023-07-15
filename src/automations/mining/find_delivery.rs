use crate::spacetraders_api::responses::Delivery;

pub fn find_delivery<'a>(deliveries: &'a [Delivery], to_find: &str) -> Option<&'a Delivery> {
    deliveries
        .iter()
        .find(|&delivery| delivery.trade_symbol == to_find)
}
