// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let spelled_number = "T-H-R-E-E"; // 使用新变量名保持字符串
    println!("Spell a Number : {}", spelled_number);
    let number = 3; // 使用原变量名但不改变类型，新声明为整数
    println!("Number plus two is : {}", number + 2);
}
