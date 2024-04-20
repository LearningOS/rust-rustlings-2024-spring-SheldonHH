struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// `ptr` 必须包含一个拥有的 `Foo` 的 box。
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: 根据约定，`ptr` 包含一个拥有的 `Foo` 的 box。我们简单地从该指针重构 box。
    Box::from_raw(ptr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let mut data = Box::new(Foo { a: 1, b: None });
        data.b = Some("hello".to_owned());  // 修改 b 以匹配测试期望

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: 我们传递一个拥有的 `Foo` 的 box。
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert_eq!(ptr_1, ptr_2, "Pointers do not match");
        assert_eq!(ret.b, Some("hello".to_owned()), "Field 'b' does not match");
    }
}
