// errors1.rs
//
// 如果你传入一个空字符串，这个函数拒绝生成名牌上要打印的文本。
// 如果能解释问题所在而不是简单地返回`None`，那会更好。
// 幸运的是，Rust 提供了一个类似于 `Result` 的结构，可以用来表达错误条件。
// 让我们使用它吧！
//
// 执行 `rustlings hint errors1` 或使用 `hint` 监视子命令来获取提示。

// 我还没做完

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 空名称是不允许的。
        Err("`name` 为空；必须是非空的。".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // 不要更改这一行
            Err("`name` 为空；必须是非空的。".into())
        );
    }
}
