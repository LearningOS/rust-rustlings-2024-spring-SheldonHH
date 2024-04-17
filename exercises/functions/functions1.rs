fn main() {
    let number = 10; // 假设这是我们要传入的参数
    call_me(number); // 正确传入参数
}

fn call_me(num: u32) {
    println!("Calling with number: {}", num);
}
