use crate::spacetraders_api::responses::{Contract, Delivery};

fn find_delivery<'a>(deliveries: &'a [Delivery], to_find: &str) -> Option<&'a Delivery> {
    deliveries
        .iter()
        .find(|&delivery| delivery.trade_symbol == to_find)
}

pub fn find_delivery_in_contracts<'a>(
    contracts: &'a [Contract],
    trade_good: &str,
) -> Option<(&'a Contract, &'a Delivery)> {
    for contract in contracts.iter() {
        if contract.accepted && !contract.fulfilled {
            if let Some(delivery) = find_delivery(&contract.terms.deliver, trade_good) {
                return Some((contract, delivery));
            }
        }
    }
    return None;
}
