// 3. traits 相关
// 3.1. 题目：什么是 trait？在 Rust 中如何定义和实现一个 trait？请为一个结构体实现一个 trait。
struct Person {
    name: String
}

trait Greet {
    fn get_greet_msg() -> String;
    fn greet(&self);
}

impl Person {
    fn new(name: String) -> Self {
        Person { name }
    }
}

impl Greet for Person {
    fn get_greet_msg() -> String {
        "nice to meet you".to_string()
    }
    fn greet(&self)  {
        println!("hey {} {}", Person::get_greet_msg(), self.name);
    }
}

fn main() {
    let _p1 = Person::new("jonathan".to_string());
    _p1.greet();
}

// 3.2. 题目：你能给出一个包含泛型的 trait 定义，并为具体类型实现它的例子吗？
// 3.3. 题目：什么是 trait bound？如何为泛型类型添加 trait bound？
// 3.4. 题目：如何使用 Default trait 为结构体提供一个默认值？
// 3.5. 题目：如何为一个结构体实现多个 trait？请给出一个例子。
