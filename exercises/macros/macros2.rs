// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// 宏定义必须放在使用之前
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 宏的调用
    my_macro!();
}
