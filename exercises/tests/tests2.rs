// tests2.rs
//
// 这个测试中有一个问题 —— 让测试编译！让测试通过！让测试失败！
//
// 执行 `rustlings hint tests2` 或使用 `hint` watch 子命令以获取提示。

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 使测试通过，可以使用两个相同的值
        assert_eq!(1, 1);

        // 为了让测试失败，你可以取消下面的注释并注释掉上面的行
        // assert_eq!(1, 2);
    }
}
