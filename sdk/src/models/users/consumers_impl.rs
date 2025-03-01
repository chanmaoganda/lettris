use crate::{Bill, Consumer, PriceCaller, RequestedGoodList};

use super::consumers::Company;

impl Consumer for Company {
    fn consumes(&self, requested_good_list: RequestedGoodList) -> Bill {
        let total_price = requested_good_list.total_price();
        Bill {
            provider: self.name.clone(),
            requested_good_list,
            total_price,
        }
    }
}
