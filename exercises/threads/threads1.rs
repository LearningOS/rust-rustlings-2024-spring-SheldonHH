// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        // 创建线程并将每个线程的句柄push到向量handles中
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis() // 返回每个线程的运行时间
        });
        handles.push(handle);
    }

    let mut results: Vec<u128> = vec![];
    // 使用join()方法等待每个线程结束，并收集返回值
    for handle in handles {
        let result = handle.join().unwrap(); // join()等待线程结束，并返回Result类型，这里使用unwrap()来获取实际值
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    // 输出每个线程的运行时间
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
