use hashbrown::HashSet;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::PriceCaller;

use super::{good_property::GoodProperty, Good};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AvailableGoodList {
    pub mappings: HashSet<GoodProperty>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Bill {
    pub mappings: HashSet<Good>,
}

impl PriceCaller for Bill {
    fn total_price(&self) -> rust_decimal::Decimal {
        self.mappings.iter()
            .fold(
                dec!(0),
                |acc, x| {
                    acc + x.total_price()
                }
            )
    }
}
