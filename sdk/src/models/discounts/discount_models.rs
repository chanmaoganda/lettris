use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum DiscountType {
    Flat,
    #[default]
    None,
    Percentage,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Discount {
    pub discount_type: DiscountType,
    pub value: Decimal,
}