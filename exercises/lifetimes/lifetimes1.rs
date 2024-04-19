// lifetimes1.rs


// 定义一个带有生命周期注释的函数，这个生命周期注释确保了输入参数和返回值的生命周期是一致的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // 调用带生命周期注释的函数
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
