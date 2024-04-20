// 在本练习中，我们给出了一个名为“numbers”的 u32 Vec，其中包含值
// 范围从 0 到 99 -- [ 0, 1, 2, ..., 98, 99 ] 我们想使用这个
// 同时在 8 个不同线程内的一组数字。 每个线程都是
// 获取每八个值的总和，并带有偏移量。
//
// 第一个线程（偏移量 0）将对 0, 8, 16, ... 求和
// 第二个线程（偏移量 1）将对 1, 9, 17, ... 求和
// 第三个线程（偏移量 2）将对 2, 10, 18, ... 求和
// ...
// 第八个线程（偏移量 7）将对 7, 15, 23, ... 求和
//
// 因为我们使用线程，所以我们的值需要是线程安全的。 所以，
// 我们正在使用 Arc。 我们需要对这两个 TODO 分别进行更改。
//
// 通过填写“shared_numbers”的值来编译此代码，其中
// 第一个 TODO 注释是，并为 `child_numbers` 创建一个初始绑定
// 第二个 TODO 注释在哪里。 尽量不要创建任何副本
// `数字` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.
#![forbid(unused_imports)] // 禁止未使用的导入，不要改动这行或下一行。
use std::sync::Arc; // 导入Arc，用于线程间共享所有权
use std::thread;

fn main() {
    let numbers: Vec<u32> = (0..100).collect();
    let shared_numbers = Arc::new(numbers); // TODO: 使用Arc包装numbers，使其可以安全地在多个线程之间共享
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers); // TODO: 为每个线程克隆Arc对象，避免数据的实际复制
        joinhandles.push(thread::spawn(move || {
            // 计算每个线程对应偏移量的数值之和
            let sum: u32 = child_numbers
                .iter()
                .enumerate()
                .filter(|&(i, _)| i % 8 == offset)
                .map(|(_, n)| n)
                .sum();
            println!("偏移量 {} 的和是 {}", offset, sum);
        }));
    }

    for handle in joinhandles {
        handle.join().unwrap(); // 确保所有线程都已经完成
    }
}
