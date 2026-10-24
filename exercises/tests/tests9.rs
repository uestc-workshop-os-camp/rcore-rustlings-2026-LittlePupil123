// tests9.rs
// Rust 非常擅长与 C/C++ 及其他静态编译语言共享 FFI 接口，它甚至可以在代码自身内部进行链接！它通过 extern
// 块来实现这一点，就像下面的代码一样。
//
// extern 关键字后面的短字符串表示外部导入的函数将遵循哪种 ABI。在这个练习中，使用的是 "Rust"，而其他变体也存在，比如
// "C" 表示标准 C ABI，"stdcall" 表示 Windows ABI。
//
// 外部导入的函数在 extern 块中声明，并用分号标记签名的结束，而不是花括号。可以对这些
// 函数声明应用一些属性来修改链接行为，例如 #[link_name = ".."] 用于
// 修改实际的符号名称。
//
// 如果你想将你的符号导出到链接环境中，也可以在函数定义前标记 extern 关键字，并带有相同的 ABI 字符串说明。Rust 函数的默认 ABI
// 字面上就是 "Rust"，所以如果你想链接纯 Rust 函数，
// 整个 extern 词都可以省略。
//
// Rust 默认会对符号进行名称修饰（mangle），就像 C++ 一样。要抑制这种行为并使
// 这些函数可通过名称寻址，可以应用属性 #[no_mangle]。
//
// 在这个练习中，你的任务是让测试用例能够调用模块 Foo 中的 my_demo_function。
// my_demo_function_alias 是 my_demo_function 的别名，所以测试用例中的两
// 行代码应该调用同一个函数。
//
// 除了添加两行属性之外，你不应该修改任何现有代码。




extern "Rust" {
    fn my_demo_function(a: u32) -> u32;

    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 外部导入的函数默认是 UNSAFE 的，
        // 因为其他语言的来源不可信。你可以
        // 将它们包装在安全的 Rust API 中，以减轻调用者的负担。
        //
        // SAFETY：我们知道这些函数是一个安全
        // Rust 函数的别名。
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
