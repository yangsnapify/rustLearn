// What does Box do in Rust?
// a box is a smart pointer help you to allocate heal with ownership and deallocation

// What is the purpose of dyn Fn?
// The purpose of dyn Fn in Rust is to represent trait objects for callable types 
// (functions, closures, etc.) that implement the Fn trait. 
// It allows you to dynamically call functions or closures without knowing their exact type at compile time

// dyn FnMut, dyn FnOnce, dyn Fn

trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    let dog: Box<dyn Speak> = Box::new(Dog);
    dog.speak();
}


// How does Box<dyn Fn> enable dynamic dispatch in Rust?
// we use trait objects to enable dynamic dispatch and when trait objects is used we use fat pointers to point tothe data and also to v table

// Can you provide an example where you use Box<dyn Fn> to store a function that has different signatures?
// its just two seperate box<dyn Fn()>
fn main() {
    let add_five: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 5);
    let multiply_by_two: Box<dyn Fn(i32) -> i32> = Box::new(|x| x * 2);

    let functions: Vec<Box<dyn Fn(i32) -> i32>> = vec![add_five, multiply_by_two];

    for func in functions {
        println!("{}", func(10));
    }
}