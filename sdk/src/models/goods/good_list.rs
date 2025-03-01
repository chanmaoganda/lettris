use hashbrown::HashSet;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::PriceCaller;

use super::{Good, GoodProperty};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AvailableGoodList {
    pub mappings: HashSet<GoodProperty>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestedGoodList {
    pub mappings: HashSet<Good>,
}

impl PriceCaller for RequestedGoodList {
    fn total_price(&self) -> rust_decimal::Decimal {
        self.mappings
            .iter()
            .fold(dec!(0), |acc, x| acc + x.total_price())
    }
}
