// as_ref_mut.rs
//
// 在你的代码中，需要给函数添加适当的 trait bound 来允许使用 as_ref 和 as_mut 方法。
// 具体来说，byte_counter 和 char_counter 函数需要一个 AsRef<str> 的约束，
// 使得可以将输入参数转换为字符串引用；而 num_sq 函数需要一个 AsMut 的约束，用于获取一个可变引用并修改它。
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.
// Obtain the number of bytes (not characters) in the given argument.
// 添加 AsRef trait 作为 trait bound
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// 添加 AsRef trait 作为 trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// 添加 AsMut trait 作为 trait bound
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // 实现函数体：计算平方
    let val = arg.as_mut();
    *val *= *val;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
