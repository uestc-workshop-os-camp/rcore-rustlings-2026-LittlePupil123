// cow1.rs
//
// 这个练习探讨 Cow，也就是 Clone-On-Write（写时克隆）类型。Cow 是一个
// 写时克隆智能指针。它可以封装借用的数据并提供不可变访问，并在
// 需要修改或需要所有权时惰性地克隆数据。该类型旨在通过
// Borrow trait 处理一般性的借用数据。
//
// 这个练习旨在向你展示在将数据传递给 Cow 时会发生什么。
// 通过在 TODO 标记处检查 Cow::Owned() 和 Cow::Borrowed()，
// 修复这些单元测试。

//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.



use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_) => Err("Expected Borrowed value"),
            _ => Ok(()),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_tmp) => Ok(()),
            _ => Err(""),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_tmp) => Ok(()),
            _ => Err("nah"),
        }
    }
}
