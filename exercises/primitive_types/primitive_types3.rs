// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// 方案2：使用范围和向量（推荐）
// 如果你想要的是一个可变长度的数组或者每个元素都不同，可以使用向量，并利用一个范围生成元素。
fn main() {
    // 使用范围创建一个包含100个连续整数的向量
    let a: Vec<i32> = (1..101).collect();  // 生成1到100的整数

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
