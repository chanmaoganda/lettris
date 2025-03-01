use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::RequestedGoodList;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Bill {
    pub provider: String,
    pub requested_good_list: RequestedGoodList,
    pub total_price: Decimal,
}
