#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 删除不安全的 unwrap() 调用
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    // 修正数组初始化，添加缺失的逗号
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 使用 clear() 方法清空向量
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 来交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
