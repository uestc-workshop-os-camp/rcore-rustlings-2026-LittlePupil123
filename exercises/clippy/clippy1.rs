// clippy1.rs
//
// Clippy 工具是一组用于分析代码的 lint，帮助你
// 发现常见错误并改进 Rust 代码。
//
// 在这些练习中，当存在 Clippy 警告时，代码将无法编译；
// 请查看输出中 Clippy 的建议来解决练习。


//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



use std::f32;

fn main() {
    let pi = std::f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
