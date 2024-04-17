// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.
fn main() {
    let data = "Rust is great!".to_string();

    // 调用 get_char 时使用引用传递
    get_char(&data);

    // 传递所有权给 string_uppercase
    string_uppercase(data);
}

// 修改为不获取所有权，只是借用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 修改为获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}
