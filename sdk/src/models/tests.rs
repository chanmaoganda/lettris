#[cfg(test)]
mod tests {
    use crate::{
        models::discounts::DiscountRule, Company, Consumer, Good, GoodProperty, NumberedGoods, RequestedGoodList, WeightedGoods
    };
    use rust_decimal_macros::dec;

    #[test]
    fn bill_test() {
        let cucumber_props = GoodProperty {
            name: "黄瓜".to_string(),
            unit_price: dec!(1.2),
            volumes: dec!(200),
            discount_rule: None,
        };
        let tomato_props = GoodProperty {
            name: "西红柿".to_string(),
            unit_price: dec!(3.2),
            volumes: dec!(200),
            discount_rule: None,
        };
        let shoe_props = GoodProperty {
            name: "鞋子".to_string(),
            unit_price: dec!(199.9),
            volumes: dec!(200),
            discount_rule: None,
        };

        let cucumber = Good::Weighted(WeightedGoods {
            property: cucumber_props,
            weight: dec!(2.33),
        });
        let tomato = Good::Weighted(WeightedGoods {
            property: tomato_props,
            weight: dec!(2.33),
        });
        let shoes = Good::Numbered(NumberedGoods {
            property: shoe_props,
            number: 2,
        });

        let request_good_list = RequestedGoodList {
            mappings: [cucumber, tomato, shoes].into_iter().collect(),
        };

        let company = Company {
            name: "拳头".to_string(),
            users: vec![],
        };

        let bill = company.consumes(request_good_list);
        assert_eq!(bill.total_price, dec!(410.052));

        dbg!(bill);
    }
}
