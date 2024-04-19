use std::num::ParseIntError;

// 定义一个函数计算购买物品的总成本
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1; // 处理费用
    let cost_per_item = 5;  // 每件物品的成本
    let qty = item_quantity.parse::<i32>()?; // 尝试将输入的字符串解析为整数

    Ok(qty * cost_per_item + processing_fee) // 如果解析成功，计算总成本并返回
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171)); // 测试有效数字
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string" // 测试无效输入
        );
    }
}
