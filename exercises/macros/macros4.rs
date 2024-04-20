// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.
// 在你的 Rust 宏定义中，存在一个常见的错误：宏的每个分支后面都需要一个分号（;）。这个符号是用来分隔不同规则的，缺少它会导致编译器错误。以下是修正后的代码：
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // 添加分号
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // 添加分号
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
