use std::borrow::Cow;
// Cow（Copy on Write）

// `abs_all`函数接受一个`Cow`类型的数组引用，并返回一个可能被修改的同类型引用。
// 这个函数将数组中所有的负数转换为其对应的正数。
fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 如果当前值为负，将其转换为正数。如果`input`是借用的，则此操作会导致数据被克隆。
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    // 测试借用数组但需要修改，应触发克隆操作。
    fn reference_mutation() -> Result<(), &'static str> {
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),  // 成功，触发了克隆，变为拥有的数据。
            _ => Err("Expected owned value"),  // 期望是拥有的值，测试失败。
        }
    }

    #[test]
    // 测试借用数组且不需要修改，不应触发克隆操作。
    fn reference_no_mutation() -> Result<(), &'static str> {
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),  // 成功，没有触发克隆，仍为借用的数据。
            _ => Err("Expected borrowed value"),  // 期望是借用的值，测试失败。
        }
    }

    #[test]
    // 测试已拥有的数组且不需要修改，不应触发克隆操作。
    fn owned_no_mutation() -> Result<(), &'static str> {
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),  // 成功，不需要修改，因此仍为拥有的数据。
            _ => Err("Expected owned value"),  // 期望是拥有的值，测试失败。
        }
    }

    #[test]
    // 测试已拥有的数组且需要修改。
    fn owned_mutation() -> Result<(), &'static str> {
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),  // 成功，即使修改了数据，但因为已经是拥有的，不影响结果。
            _ => Err("Expected owned value"),  // 期望是拥有的值，测试失败。
        }
    }
}
