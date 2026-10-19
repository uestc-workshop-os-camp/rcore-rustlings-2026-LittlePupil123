// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}



// 步骤：
// 1. 如果提供的字符串长度为 0，则应返回一个错误
// 2. 按字符串中出现的逗号进行分割
// 3. 分割结果只应返回 2 个元素，否则返回一个
// 错误
// 4. 从分割操作中取出第一个元素，并将其用作 name
// 5. 从分割操作中取出另一个元素，并使用类似 "4".parse::<usize>() 的方式
// 将其解析为 usize 作为 age
// 6. 如果在提取 name 和 age 时出现问题，则应返回一个
// 错误
// 如果一切顺利，则返回一个包含 Person 对象的 Result
//
// 附带说明：Box<dyn Error> 实现了 From<&'_ str>。这意味着，如果你
// 想返回一个字符串错误消息，可以直接使用
// Err("my error message".into()) 来实现。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {return Err(Self::Err::Empty)};
        
        let mut s_iter = s.split(',');
        let len = s_iter.clone().count();

        if len != 2 {return Err(Self::Err::BadLen)};

        if let Some(name) = s_iter.next() {
            if name.is_empty() {return Err(Self::Err::NoName)};
            if let Some(age) = s_iter.next() {
                match age.parse::<usize>(){
                    Ok(age) => return Ok(Person{name:name.to_string(),age}),
                    Err(reason) => return Err(Self::Err::ParseInt(reason)),
                }
            }
        }
        Err(Self::Err::BadLen)
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
