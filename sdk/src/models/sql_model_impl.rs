use crate::FromCustomRow;

use super::{DiscountRule, DiscountType, GoodProperty, GoodPropertyRow};

impl FromCustomRow<GoodPropertyRow> for GoodProperty {
    fn from_custom_row(row: GoodPropertyRow) -> GoodProperty {
        let GoodPropertyRow {
            name,
            price,
            volumes,
            discount_type,
            discount_value,
        } = row;
        let discount_rule = if discount_type.is_some() && discount_value.is_some() {
            Some(
                DiscountRule {
                    discount_type: DiscountType::from(discount_type.unwrap()),
                    value: discount_value.unwrap(),
                }
            )
        } else { None };
        GoodProperty {
            name,
            unit_price: price,
            volumes,
            discount_rule,
        }
    }
}