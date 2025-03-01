#[cfg(test)]
mod tests {
    use crate::{Bill, Good, GoodProperty, PriceCaller, WeightedGoods, NumberedGoods};
    use rust_decimal_macros::dec;

    #[test]
    fn bill_test() {
        let cucumber_props = GoodProperty { name: "黄瓜".to_string(), unit_price: dec!(1.2) };
        let tomato_props = GoodProperty { name: "西红柿".to_string(), unit_price: dec!(3.2) };
        let shoe_props = GoodProperty { name: "鞋子".to_string(), unit_price: dec!(199.9) };

        let cucumber = Good::Weighted(WeightedGoods { property: cucumber_props, weight: dec!(2.33) });
        let tomato = Good::Weighted(WeightedGoods { property: tomato_props, weight: dec!(2.33) });
        let shoes = Good::Numbered(NumberedGoods {property: shoe_props, number: 2});

        let bill = Bill {
            mappings: [cucumber, tomato, shoes].into_iter().collect(),
        };

        assert_eq!(bill.total_price(), dec!(410.052));
    }
}