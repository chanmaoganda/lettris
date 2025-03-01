use rust_decimal::{Decimal, prelude::FromPrimitive};
use serde::{Deserialize, Serialize};

use crate::{DiscountCaller, PriceCaller};

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

impl<F> DiscountCaller<F> for WeightedGoods
where 
    F: Fn(Decimal) -> Decimal
{
    fn discount_price(&self, f: F) -> Decimal {
        f(self.weight)
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
        debug_assert!(
            number.is_some(),
            "{}, number cannot be converted to rust_decimal::Decimal",
            line!()
        );
        let number = number.unwrap();
        number * self.property.unit_price
    }
}

impl<F> DiscountCaller<F> for NumberedGoods
where 
    F: Fn(i32) -> Decimal
{
    fn discount_price(&self, f: F) -> Decimal {
        f(self.number)
    }
}

/// FIXME: make good available with weighted and numbered
#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Good {
    Weighted(WeightedGoods),
    Numbered(NumberedGoods),
}

impl PriceCaller for Good {
    fn total_price(&self) -> Decimal {
        match self {
            Self::Weighted(good) => good.total_price(),
            Self::Numbered(good) => good.total_price(),
        }
    }
}
