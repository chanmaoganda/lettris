use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(sqlx::FromRow, Deserialize)]
pub struct GoodPropertyRow {
    pub name: String,
    pub price: Decimal,
    pub volumes: Decimal,
    pub discount_type: Option<String>,
    pub discount_value: Option<Decimal>,
}
