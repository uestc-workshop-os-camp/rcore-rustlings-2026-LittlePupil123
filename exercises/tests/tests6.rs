// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize; //ptr1存data的a字段内存地址的名称
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) }; //将data传进去，在另一个函数内部通过指针对data做操作

        let ptr_2 = &ret.a as *const u128 as usize; //ptr2存ret的字段，由于地址不变，于是ret.a实际上就是data.a

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
