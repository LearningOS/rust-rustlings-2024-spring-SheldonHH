// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // 在这里不再需要初始化 vec0
    let mut vec1 = fill_vec();  // 直接调用修改后的 fill_vec，它不需要参数

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` 不再接受参数，而是在函数内部创建和返回一个 Vec<i32>
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();  // 正确初始化一个空的向量

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
