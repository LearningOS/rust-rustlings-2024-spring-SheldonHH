fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // 使用 `as` 操作符将 values.len() 转换为 f64 类型
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
