mod discount_models;

use hashbrown::{HashMap, HashSet};
use rust_decimal::Decimal;

use super:: GoodProperty;

pub use discount_models::*;

/// Calculate the final discount
/// 
/// WARN: Checks the discount level to avoid invalid discount calculation
/// 
/// WARN: Consider all possible conditions
/// 1. discount for a special single item -> Tags::SpecialDiscount
/// 2. discount for a kind of items -> Tags::Discount
/// 3. no discount for a kind of items -> Tags::Common
/// 
/// However, discount for a kind may contains many different kinds of discounts
/// 
/// TODO: We can implement this later
pub struct DiscountMgr<F>
{
    pub mappings: HashSet<F>,
    // pub tags: HashMap<Tags, HashSet<GoodProperty>>,
}

impl<F> DiscountMgr<F>
where 
    F: Fn(Decimal, Decimal) -> Decimal
{
    pub fn new() -> Self {
        Self {
            mappings: HashSet::new()
        }
    }

    pub fn total_discount() {

    }
}

// pub enum Discount {
//     Fixed(Decimal),
//     Percentage(Decimal),
// }

// pub enum DiscountTrigger {

// }
