/*
在你提供的 Rust 程序中，编译器错误指出你忘记填写 Box<dyn ???>> 中的类型标识符。你的 main 函数的返回类型应该是一个 Box，这个 Box 包含一个动态分配的错误类型。这个错误类型应该实现了 Rust 的 error::Error 特性，这样才能覆盖所有可能的错误类型。

你的程序中存在两种错误类型：

ParseIntError —— 当字符串转换为整数时可能发生。
CreationError —— 当尝试创建 PositiveNonzeroInteger 时，如果数值是负数或零。
为了使得 main 函数能够正确处理这两种错误类型，你需要确保 main 函数的返回类型为 Result<(), Box<dyn error::Error>>，这表明该函数返回的错误可以是任何实现了 error::Error 特性的类型。




*/
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}
