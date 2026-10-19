// from_into.rs
//
// From trait 用于值到值的转换。如果为某个类型正确实现了 From，
// 那么 Into trait 应该能反向工作。你可以在
// https://doc.rust-lang.org/std/convert/trait.From.html 阅读更多关于它的内容
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 你的任务是完成这个实现，以便让 let p = // Person::from("Mark,20") 这一行能够编译。请注意，你需要将
// age 部分解析为 usize，可以使用类似 "4".parse::<usize>() 的方式。这个
// 解析结果需要被妥善处理。
//
// 步骤：
// 1. 如果提供的字符串长度为 0，则返回 Person 的默认值。
// 2. 按字符串中出现的逗号进行分割。
// 3. 从分割结果中取出第一个元素，并将其用作 name。
// 4. 如果 name 为空，则返回 Person 的默认值。
// 5. 从分割结果中取出另一个元素，并将其解析为
// usize 作为 age。
// 如果在解析 age 时出现问题，则返回 Person 的默认值。
// 否则，返回一个用这些结果实例化的 Person 对象。



impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let mut s_iter = s.split(',');
        let len = s_iter.clone().count();
        if len==0{
            Person::default()
        }else if len==2{
            if let (name,age) = (s_iter.next(),s_iter.next()){
                match name {
                    Some(name) => {
                        if name.is_empty() {
                            return Person::default();
                        }
                        if let Some(age) = age{
                            match age.parse::<usize>() {
                                Ok(age) => return Person{name:name.to_string(),age},
                                Err(_) => return Person::default(),
                            }
                        }else{Person::default()}
                    },
                    None => Person::default(),
                }
            }else{Person::default()}

        }else{Person::default()}
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
