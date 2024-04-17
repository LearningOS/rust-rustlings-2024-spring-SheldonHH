#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    // 构造函数，验证包裹的重量必须大于0，否则抛出异常
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    // 判断包裹是否为国际包裹，不同国家返回true
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    // 计算运输费用，费用基于每克的成本乘以包裹重量
    fn get_fees(&self, cents_per_gram: i32) -> i32 {
        cents_per_gram * self.weight_in_grams
    }
}