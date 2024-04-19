trait AppendBar {
    fn append_bar(self) -> Self;
}

// 这段代码实现了 AppendBar trait，确保了 Vec<String> 可以使用 append_bar 方法。
// 这个方法会把一个新的 String 对象 "Bar" 添加到向量的末尾，并返回这个修改后的向量。

// 现在，你的测试 is_vec_pop_eq_bar 应该能够正确执行，
// 首先移除并检查 "Bar"，然后是 "Foo"。这样一来
// ，你的代码应该不再出现之前的编译错误。
// 为 Vec<String> 实现 AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
