// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}


// 将 &str 转换为 String，但它们的适用场景略有不同：

// 使用 .to_owned() 时，你明确知道需要一个拥有所有权的副本，
// 通常用于字符串和其他集合数据类型。
// 使用 .into() 时，通常是在类型之间的转换需要自动进行且你不想明确指定类型的场景，
// 这使得 .into() 在泛型编程中特别有用。
fn main() {
    string_slice("blue");                             // &str 直接传递
    // 使用 .to_string(), String::from(), .to_owned(), .into(), 和 format!() 方法创建的字符串都是 String 类型，
    // 所以需要用 string 函数。
    string("red".to_string());                        // String
    string(String::from("hi"));                       // String
    string("rust is fun!".to_owned());                // String
    string("nice weather".into());                    // String
    string(format!("Interpolation {}", "Station"));   // String
    string_slice(&String::from("abc")[0..1]);         // &str (切片)
    string_slice("  hello there ".trim());            // &str (trim 返回 &str)
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // String (replace 返回 String)
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());  // String (to_lowercase 返回 String)
}

