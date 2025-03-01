use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};

use crate::PriceCaller;

use super::good_property::GoodProperty;

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct WeightedGoods {
    pub property: GoodProperty,
    pub weight: Decimal,
}

impl PriceCaller for WeightedGoods {
    fn total_price(&self) -> Decimal {
        self.weight * self.property.unit_price
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct NumberedGoods {
    pub property: GoodProperty,
    // WARN: potential overflow? not that possible?
    pub number: i32,
}

impl PriceCaller for NumberedGoods {
    fn total_price(&self) -> Decimal {
        let number = Decimal::from_i32(self.number);
        debug_assert!(number.is_some(), "{}, number cannot be converted to rust_decimal::Decimal", line!());
        let number = number.unwrap();
        number * self.property.unit_price
    }
}
