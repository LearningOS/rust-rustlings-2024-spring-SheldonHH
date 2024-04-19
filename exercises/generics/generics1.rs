// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


// 选择使用 String 还是 &'static str 取决于你的具体需求。如果你的字符串不会在运行时改变或不需要拥有字符串，使用 &'static str 足够且更为高效。如果你需要在运行时修改字符串或需要从其他地方动态获取字符串，使用 String 更合适。



fn main() {
    let mut shopping_list: Vec<&'static str> = Vec::new();
    shopping_list.push("milk");  // 直接使用字符串字面量
}
