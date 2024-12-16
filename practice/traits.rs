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
trait Sum<T> {
    fn sum(v1: T, v2: T) -> T;
}

impl Sum<i32> for i32 {
    fn sum(v1: i32, v2:i32) -> i32 {
        v1 + v2
    }
}

fn main() {
    i32::sum(1, 2);
}

// 3.3. 题目：什么是 trait bound？如何为泛型类型添加 trait bound？
// traits bound can be genercis trait bound or where claused trait bound
// traits first arguments although is self but its actually means left hand side for some special traits fn like add, mul
// move || => create a closures and also forced ownership transferred

use std::fmt::Display;

fn print_value<T: Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    let x = 42;
    print_value(x);
}



// 3.4. 题目：如何使用 Default trait 为结构体提供一个默认值？
// ..Default::default() change the rest of the keys
// because rust prevent you override some types(primitive) so you can either create your own types(struct) to override the default

struct MyI32(i32);

struct SomeKeys {
    key: i32,
    other_value: i32,
}

impl Default for SomeKeys {
    fn default() -> Self {
        SomeKeys {
            key: 1,
            other_value: 0,
        }
    }
}

impl SomeKeys {
    fn new(other_value: i32) -> SomeKeys {
        SomeKeys {
            key: Self::default().key,
            other_value,
        }
    }
}

fn main() {
    let p1 = SomeKeys::new(2);
    println!("key: {}, other_value: {}", p1.key, p1.other_value);
}


// 3.5. 题目：如何为一个结构体实现多个 trait？请给出一个例子。
use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle: {}x{}", self.width, self.height)
    }
}

trait Area {
    fn area(&self) -> u32;
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

trait Perimeter {
    fn perimeter(&self) -> u32;
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 5,
        height: 10,
    };

    println!("{}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}

// deref
// impl structorenum
// generics struct impl<T> Somestruct<T>
// traits impl traits1 for structs
// generics traits impl Sum<i32> for i32 
// traits bound => function based or where clause
// move ||