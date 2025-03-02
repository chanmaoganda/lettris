use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::discounts::Discount;

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq, FromRow)]
pub struct GoodProperty {
    pub name: String,
    // NOTICE: this price may be like `12.33 per gram`, `12.44 per unit`, etc.
    pub unit_price: Decimal,
    #[sqlx(json)]
    pub discount: Discount,
}
