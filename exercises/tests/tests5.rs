// 要解决这个问题，我们需要填写 modify_by_address 函数的内容，
// 并确保符合安全约定。
// 这个函数的目的是通过一个 usize 类型的地址修改一个 u32 类型的值。考虑到 Rust 的所有权和借用规则，这种操作是不安全的，因此必须使用 unsafe 块。

// 根据测试用例的描述，我们需要修改这个 u32 的值为 0xAABBCCDD。
// 这里是一个安全的实现方法，它遵循了测试注释中的安全标准：
/// # Safety
///
/// The caller must ensure that the `address` points to a valid, mutable
/// location of a `u32`. The caller must also ensure that there are no other
/// references to this `u32` for the duration of the call, to uphold Rust's
/// aliasing rules.
unsafe fn modify_by_address(address: usize) {
    // 将 usize 地址转换回 *mut u32 指针
    let ptr = address as *mut u32;
    // 修改 *mut u32 指向的值为 0xAABBCCDD
    *ptr = 0xAABBCCDD;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD);
    }
}
