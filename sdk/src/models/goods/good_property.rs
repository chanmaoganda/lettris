use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct GoodProperty {
    pub name: String,
    // NOTICE: this price may be like `12.33 per gram`, `12.44 per unit`, etc.
    pub unit_price: Decimal,
}
