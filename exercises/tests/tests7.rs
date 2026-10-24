// tests7.rs
//
// 在构建包时，有些依赖既不能在 Cargo.toml 中导入，也不能直接链接；
// 有些预处理过程会因代码生成方式不同而变化，用于设置包特定的配置。
//
// Cargo 并不打算取代其他构建工具，但它通过名为 build.rs 的自定义构建脚本
// 与它们集成。这个文件通常放在项目根目录，而在本例中，它放在
// 这个练习所在的同一目录下。
//
// 它可以用于：
//
// - 构建一个捆绑的 C 库。
// - 在宿主系统上查找一个 C 库。
// - 根据规格说明生成一个 Rust 模块。
// - 执行该 crate 所需的任何平台特定配置。
//
// 在设置配置时，我们可以在构建脚本中使用 println!
// 来告诉 Cargo 遵循某些指令。通用格式是：
//
// println!("cargo:{}", your_command_in_string); {}占位的部分是key=value
//
// 请参阅 Cargo 官方手册中关于构建脚本的更多
// 信息：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在这个练习中，我们查找一个环境变量，并期望它
// 落在某个范围内。你可以查看测试用例来了解细节。
//
// 你不应该修改这个文件。请修改同一目录下的 build.rs
// 来通过这个练习。
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.



fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
