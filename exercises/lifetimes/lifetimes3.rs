// 定义一个包含生命周期注释的结构体 `Book`
struct Book<'a> {
    author: &'a str,  // 添加生命周期注释，表示 `author` 引用的数据至少与 `Book` 实例保持相同的生命周期
    title: &'a str,   // 添加生命周期注释，表示 `title` 引用的数据至少与 `Book` 实例保持相同的生命周期
}

fn main() {
    let name = String::from("Jill Smith");  // `name` 是一个 String 实例
    let title = String::from("Fish Flying");  // `title` 也是一个 String 实例
    let book = Book { author: &name, title: &title };  // 创建 `Book` 实例时，传入 `name` 和 `title` 的引用

    println!("{} by {}", book.title, book.author);  // 打印书名和作者
}
