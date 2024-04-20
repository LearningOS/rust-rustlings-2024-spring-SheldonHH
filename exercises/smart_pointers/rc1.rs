// rc1.rs
//
// 在这个练习中，我们想通过使用 Rc<T> 类型来表达多重所有权的概念。这是我们太阳系的一个模型 —— 有一个 Sun 类型和多个 Planet。
// Planet 拥有对 sun 的所有权，表示它们绕太阳旋转。
//
// 通过适当使用 Rc 类型的方法使代码能编译通过。
//
// 执行 `rustlings hint rc1` 或使用 `hint` 监视子命令获取提示。

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 1 引用

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 2 引用
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 3 引用
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 4 引用
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 5 引用
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 6 引用
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 7 引用
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 8 引用
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 9 引用
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    // 逐一删除 planet，观察引用计数的变化
    drop(neptune);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 8 引用

    drop(uranus);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 7 引用

    drop(saturn);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 6 引用

    drop(jupiter);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 5 引用

    drop(mars);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 4 引用

    drop(earth);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 3 引用

    drop(venus);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 2 引用

    drop(mercury);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 1 引用

    assert_eq!(Rc::strong_count(&sun), 1);
}
