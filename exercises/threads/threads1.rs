// threads1.rs
//
// 这个程序会生成多个线程，每个线程至少运行 250ms，并且
// 每个线程都会返回它们完成所需的时间。程序应该
// 等待所有已生成的线程都结束，并应将它们的
// 返回值收集到一个向量中。
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.



use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO：thread::spawn 会返回一个结构体，你能使用它吗？
        let data=handle.join().unwrap();//子线程的返回值在调用join后先返还给data存住，再给到res
        results.push(data);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
