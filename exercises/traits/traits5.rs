pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 修改：使用泛型和 trait bounds 约束 item 必须同时实现 SomeTrait 和 OtherTrait
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()  // 使用两个 trait 中的方法
}

fn main() {
    some_func(SomeStruct {});   // 正确调用，因为 SomeStruct 实现了这两个 trait
    some_func(OtherStruct {});  // 正确调用，因为 OtherStruct 也实现了这两个 trait
}
