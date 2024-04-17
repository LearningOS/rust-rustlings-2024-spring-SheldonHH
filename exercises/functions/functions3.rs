// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let number = 10; // 假设这是我们要传入的参数
    call_me(number); // 正确传入参数
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
