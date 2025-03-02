use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum DiscountType {
    #[default]
    Flat,
    Percentage,
}

impl From<String> for DiscountType {
    fn from(value: String) -> Self {
        if value == "Flat" {
            Self::Flat
        } else if value == "Percentage" {
            Self::Percentage
        } else {
            panic!("no matching conversion for `{}` to DiscountType", value);
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct DiscountRule {
    pub discount_type: DiscountType,
    pub value: Decimal,
}