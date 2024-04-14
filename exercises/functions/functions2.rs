// functions2.rs

fn main() {
    call_me(3);
}

// Properly type the parameter `num`
fn call_me(num: usize) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
