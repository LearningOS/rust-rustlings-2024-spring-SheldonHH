// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types2` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // 字符（`char`）

    // 注意使用的是单引号，这和你之前看到的双引号不同。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // 完成这一行，就像示例中的那样！你最喜欢的字符是什么？
    // 试试字母，试试数字，试试特殊字符，试试不同语言的字符，试试表情符号！
    let your_character = '🔥'; // 这里我选择了一个特殊字符作为示例
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
