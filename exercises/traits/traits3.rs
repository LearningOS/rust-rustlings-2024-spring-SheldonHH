// traits3.rs
//
// 您的任务是实现两个结构的 Licensed 特征trait并具有
// 它们返回相同的信息，而无需两次编写相同的函数。
//
// 考虑可以添加到许可特征trait中的内容
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

pub trait Licensed {
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

// 实现 Licensed trait 的 licensing_info 方法
impl Licensed for SomeSoftware {
    fn licensing_info(&self) -> String {
        "Some information".to_string()
    }
}

impl Licensed for OtherSoftware {
    fn licensing_info(&self) -> String {
        "Some information".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
