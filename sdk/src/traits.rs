/// FIXME: These traits should as much as possible situations to cover all needs for all parties
///
/// FIXME: All users should be set a privilege for potential service stress in considerations
use rust_decimal::Decimal;

use crate::{Bill, RequestedGoodList};

/// TODO: this type is for different kinds of users,
///
/// Potential users are:
/// Company? Organization? School? Government?
pub trait Consumer {
    fn consumes(&self, good_list: RequestedGoodList) -> Bill;
}

/// TODO: this type is for different kinds of producers,
///
/// Potential users are:
/// Farmers? Family-based-organization? Private Salary Provider? Official Salary Provider?
pub trait Producer {
    fn produces(&self, good_list: &RequestedGoodList);
}

pub trait PriceCaller {
    fn total_price(&self) -> Decimal;
}

pub trait DiscountCaller<F> {
    fn discount_price(&self, f: F) -> Decimal;
}