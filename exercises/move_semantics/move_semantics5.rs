fn main() {
    let mut x = 100;
    {
        let y = &mut x; // `y` 的作用域开始
        *y += 100;     // 使用 `y` 进行修改
    } // `y` 的作用域结束

    let z = &mut x; // 现在可以安全地创建 `z`，因为 `y` 已经不再使用
    *z += 1000;
    
    assert_eq!(x, 1200);
}
