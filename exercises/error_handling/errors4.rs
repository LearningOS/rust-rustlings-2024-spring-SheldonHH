use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {  // 修改main函数的返回类型
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;  // 使用?操作符处理错误

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())  // 正常结束时返回Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;  // 解析字符串为整数

    Ok(qty * cost_per_item + processing_fee)  // 返回计算的总费用
}
