#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("这是一个空的链表: {:?}", create_empty_list());
    println!(
        "这是一个非空的链表: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil // 返回 Nil 变体表示一个空链表
}

pub fn create_non_empty_list() -> List {
    // 返回一个非空链表，例如：Cons(42, Box::new(Nil))
    List::Cons(42, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list());
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
