use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::PriceCaller;

mod good_models;
mod good_property;
mod good_list;

pub use good_list::{AvailableGoodList, Bill};
pub use good_property::GoodProperty;
pub use good_models::{NumberedGoods, WeightedGoods};

/// FIXME: make good available with weighted and numbered
#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Good {
    Weighted(WeightedGoods),
    Numbered(NumberedGoods),
}

impl PriceCaller for Good {
    fn total_price(&self) -> Decimal {
        match self {
            Self::Weighted(good) => {
                good.total_price()
            }
            Self::Numbered(good) => {
                good.total_price()
            }
        }
    }
}