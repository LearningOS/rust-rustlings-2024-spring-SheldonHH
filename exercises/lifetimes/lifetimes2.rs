fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 返回两个字符串切片中较长的那个
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // 在与 `string1` 相同的作用域中声明 `string2`
    let string2 = String::from("xyz");
    // 现在 `string1` 和 `string2` 拥有相同的生命周期作用域
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
