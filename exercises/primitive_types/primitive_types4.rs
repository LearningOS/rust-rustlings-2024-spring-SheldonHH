#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // 获取数组 `a` 的从第二个元素（索引 1）开始到第四个元素（索引 3）结束的切片
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice);
}
