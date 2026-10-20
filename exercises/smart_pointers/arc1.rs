// arc1.rs
//
// 在这个练习中，我们得到一个名为 "numbers" 的 u32 的 Vec，其值
// 范围从 0 到 99 —— [ 0, 1, 2, ..., 98, 99 ]。我们想同时
// 在 8 个不同的线程中使用这组数字。每个线程
// 将获取每隔八个值的和，并带有一个偏移量。
//
// 第一个线程（偏移量 0）将对 0、8、16、... 求和
// 第二个线程（偏移量 1）将对 1、9、17、... 求和
// 第三个线程（偏移量 2）将对 2、10、18、... 求和
// ...
// 第八个线程（偏移量 7）将对 7、15、23、... 求和
//
// 因为我们使用线程，所以我们的值需要是线程安全的。因此，
// 我们使用 Arc。我们需要在两个 TODO 中各做一处修改。
//
// 通过在第一个 TODO 注释处为 shared_numbers 填入一个值，
// 并在第二个 TODO 注释处为 child_numbers 创建一个初始绑定，
// 使这段代码能够编译。尽量不要创建 numbers Vec 的任何副本！
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.



#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
