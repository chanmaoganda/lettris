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
    F: Fn(Decimal, Decimal) -> Decimal
{
    fn discount_price(&self, f: F) -> Decimal {
        f(self.property.unit_price, self.weight)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct NumberedGoods {
    pub property: GoodProperty,
    // WARN: potential overflow? not that possible?
    pub number: i32,
}

impl NumberedGoods {
    pub fn decimal_number(&self) -> Decimal {
        let number = Decimal::from_i32(self.number);
        debug_assert!(
            number.is_some(),
            "{}, number cannot be converted to rust_decimal::Decimal",
            line!()
        );
        number.unwrap()
    }
}

impl PriceCaller for NumberedGoods {
    fn total_price(&self) -> Decimal {
        let number = self.decimal_number();
        number * self.property.unit_price
    }
}

impl<F> DiscountCaller<F> for NumberedGoods
where 
    F: Fn(Decimal, Decimal) -> Decimal
{
    fn discount_price(&self, f: F) -> Decimal {
        f(self.property.unit_price, self.decimal_number())
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

impl<F> DiscountCaller<F> for Good
where
    F: Fn(Decimal, Decimal) -> Decimal,
{
    fn discount_price(&self, f: F) -> Decimal {
        match self {
            Self::Weighted(good) => good.discount_price(f),
            Self::Numbered(good) => good.discount_price(f),
        }
    }
}
