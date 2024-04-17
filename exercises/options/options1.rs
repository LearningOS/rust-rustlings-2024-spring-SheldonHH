// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// 这个函数返回冰箱里剩余的冰激凌数量。
// 如果是晚上10点前，有5块剩余。到了10点，有人把它们吃掉了，
// 所以之后就没有剩余了 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 我们这里使用24小时制，所以晚上10点是22，午夜12点是0
    // Option输出应该优雅地处理time_of_day > 23的情况。
    if time_of_day > 23 {
        None
    } else if time_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
