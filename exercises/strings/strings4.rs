// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi")); 
    string("rust is fun!".to_owned()); //to_owned代表“复制”，to_string代表“转换”（实际是复制，但更偏向格式化字面值并转换）,二者都变为字符串
    string("nice weather".into()); //into完成到string的转换（真正意义上的，所有权也会发生转移)
    string(format!("Interpolation {}", "Station")); //format!拼接会返回一个字符串类型
    string_slice(&String::from("abc")[0..1]); //切片生成
    string_slice("  hello there ".trim()); //trim返回一个&str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); //replace会返回一个string类型
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); //返回string，表示复制一份，并将大写全部换为小写
}
