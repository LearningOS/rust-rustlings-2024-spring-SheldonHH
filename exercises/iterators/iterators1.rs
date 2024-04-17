// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Step 1: 创建一个可迭代的引用
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // Step 2: 迭代到第二个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // Step 3: 迭代到第四个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // Step 4: 迭代完成，返回 None
}
