pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 定义transformer函数，该函数接受一个包含(String, Command)元组的向量作为输入，并返回一个字符串向量
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();  // 初始化一个空的字符串向量用于存储结果

        // 遍历input向量。之前代码中使用iter()和ref mut导致无法修改string，
        // 现在改用into_iter()来消费input，从而可以直接获取并修改元组中的String
        for (mut string, command) in input.into_iter() {  
            match command {
                // 对字符串执行大写转换
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                },
                // 对字符串执行裁剪操作，移除前后空白
                Command::Trim => {
                    output.push(string.trim().to_string());
                },
                // 对字符串执行追加操作，追加指定次数的"bar"
                Command::Append(n) => {
                    let append_str = "bar".repeat(n);  // 根据n的值重复"bar"字符串
                    string.push_str(&append_str);  // 将重复后的字符串追加到原字符串
                    output.push(string.clone());  // 将修改后的字符串添加到输出向量
                }
            }
        }
        output  // 返回包含处理后的字符串的向量
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        // 测试transformer函数，确保它按预期工作
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
