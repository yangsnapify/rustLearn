// 2. impl 相关
// 2.1. 题目：如何为一个结构体实现一个 new 函数，作为构造函数？
struct Person {
    name: String;
    age: i32;
}

impl Person {
    fn new(_name: String, _age: i32) -> Self {
        Person { name: _name, age: _age }
    }
    fn get_name(&self) {
        println!("{}", self.name)
    }
}

fn main() {
    let _p1 = Person::new("jonathan".to_string(), 18);
}
// the arguements for the method can be self, &self, &mut self
// use self when you want to consume ownership
// use &self when you want to not consume ownership
// use &mut self when you want to change some attributes


// 2.2. 题目：如何为一个结构体实现一个方法，使其能够返回结构体的某个字段的值？
struct Car {
    pub model_name: String;
    pub count_of_tyres: i32;
}

impl Car {
    fn new (_model_name: String, _count_of_tyres: i32) -> Self {
        Car{ model_name: _model_name, count_of_tyres: _count_of_tyres }
    }
    fn get_model_name(&self) -> &String {
        self.model_name
    }
    fn update_count_of_tyres(&mut self, count: i32) {
        self.count_of_tyres = count;
    }
}

fn main() {
    let mut _p1 = Car::new("toyota".to_string(), 6);
    _p1.update_count_of_tyres(4);
    println("{} {}", _p1.model_name, _p1.count_of_tyres);
}
// by default structs attrbites is private means you cannot access using println, you need to either use gettersetters or pub
// in some cases when usine Option<T> you have to remember using as_ref because if not you will consuming option<T> ownership

// 2.3. 题目：你能为一个包含泛型类型的结构体实现方法吗？给出一个例子。
struct Datatypes<T> {
    name: T;
}

impl<T> Datatypes<T> {
    fn new(name: T) -> Self {
        Datatypes {
            name
        }
    }
}
fn main() {
    let _d1 = Datatypes::new("number".to_string() );
    let _d2 Datatypes::new(43)
}

// 2.4. 题目：解释 impl 和 impl<T> 的区别，分别在什么情况下使用？
// the different is about concrete type vs generics type
// concrete types means that we know the actual types
// generics types means that are multiple types we need to implement