// 这是您定义的函数，用于判断一个整数是否为偶数
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

// 这是测试模块，包含了两个测试用例
#[cfg(test)]
mod tests {
    use super::*;

    // 测试用例：当输入偶数时，`is_even` 应该返回 true
    #[test]
    fn is_true_when_even() {
        // 选择一个偶数来测试，这里我们使用 4
        assert!(is_even(4), "4 should be even");
    }

    // 测试用例：当输入奇数时，`is_even` 应该返回 false
    #[test]
    fn is_false_when_odd() {
        // 选择一个奇数来测试，这里我们使用 5
        assert!(!is_even(5), "5 should be odd");
    }
}
