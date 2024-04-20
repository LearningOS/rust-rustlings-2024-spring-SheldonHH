// 在 Rust 中，宏的作用域处理略有不同于函数。宏需要在它们被调用之前声明，或者通过适当的方式使其在调用点可见。在你提供的代码示例中，my_macro 宏在 macros 模块内部定义，但在模块外部的 main 函数中调用。这导致编译错误，因为 main 函数中无法找到宏。

// 为了解决这个问题，你需要使用 #[macro_export] 属性来标记宏，这样它就可以在定义它的模块之外使用了。这个属性确保宏在整个包中都是可见的。下面是修改后的代码：


mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
