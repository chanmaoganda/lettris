use hashbrown::{HashMap, HashSet};

use super::{ GoodProperty, Tags};

/// Calculate the final discount
/// 
/// WARN: Checks the discount level to avoid invalid discount calculation
pub struct DiscountMgr<F>
{
    pub mappings: HashMap<GoodProperty, F>,
    pub tags: HashSet<Tags, Vec<GoodProperty>>,
}

impl<F> DiscountMgr<F> {

}
