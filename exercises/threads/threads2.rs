use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}
/*
在你的多线程Rust程序中，JobStatus 结构体被用于共享状态，
但Rust的所有权和并发规则要求更安全的访问控制。
因为 jobs_completed 字段需要在多个线程中被修改，
我们需要使用互斥锁（Mutex）来确保线程安全。

此外，要正确打印出共享状态的值，我们应在所有线程完成后再打印，而不是在每个线程结束时立即打印。
这样可以避免读取到部分完成的状态。
*/
fn main() {
    // 使用Mutex包装JobStatus来确保线程安全
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 获取锁并更新jobs_completed
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 在所有线程完成后打印最终的jobs_completed的值
    let completed = status.lock().unwrap().jobs_completed;
    println!("jobs completed {}", completed);
    // 期望输出是“jobs completed 10”，因为所有10个线程均已完成工作。
}
