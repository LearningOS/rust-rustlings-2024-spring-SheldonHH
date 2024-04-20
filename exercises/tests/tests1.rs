// 这是一个测试模块，包含了测试用例
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        // 使用 assert! 宏来确保某个条件为真
        // 此处我们使测试通过，表达式为 true
        assert!(true, "This test should pass");

        // 若要使测试失败，可以将上一行代码改为：
        // assert!(false, "This test should fail");
    }
}
