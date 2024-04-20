// tests4.rs
//
// 确保我们在测试中检查了正确的条件！

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // 只更改测试函数本身
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 这个测试应该检查我们传入构造函数的矩形尺寸是否正确
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽度
        assert_eq!(rect.height, 20); // 检查高度
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        // 这个测试应该检查当我们尝试使用负宽度创建矩形时程序是否会panic
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        // 这个测试应该检查当我们尝试使用负高度创建矩形时程序是否会panic
        let _rect = Rectangle::new(10, -10);
    }
}
