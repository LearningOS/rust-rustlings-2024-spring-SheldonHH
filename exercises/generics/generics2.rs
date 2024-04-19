struct Wrapper<T> {  // 使用泛型参数 T 定义结构体
    value: T,
}

impl<T> Wrapper<T> {  // 使用泛型参数 T 实现方法
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);  // 测试存储 u32 类型
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");  // 测试存储 &str 类型
    }
}
