// 1. 基础数据结构
// 1.1. 题目：解释 Rust 中的 struct 和 enum，并给出一个实际应用场景。
enum MaritalStatus {
    Single,
    Married,
    Divorced
}

struct Person {
    name: String,
    age: u32,
    status: MaritalStatus
}

fn main() {
   let _p1 = Person{name: "jonathan".to_string(), age: 100, status: MaritalStatus::Single};
}

// 1.2. 题目：如何在 Rust 中创建一个数组，并遍历其元素？
fn main() {
    let _v1: [i32; 3] = [1, 2, 3];
    let _v2: Vec<i32> = vec![1, 2, 3];

    for &item in _v1.iter() {
        println!("{}", item);
    }
}

// 1.3. 题目：在 Rust 中，如何创建一个固定大小的数组，如何修改其中的元素？
fn main() {
    let mut _v1: [i32; 3] = [1, 2, 3];
    _v1[1] = 5;
    println!("{:?}", _v1);
}

// 1.4. 题目：解释 Rust 中的 切片（slice） 和 数组 有什么不同？给出一个例子。
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &arr[1..3]; 
    println!("slice: {:?}", slice);
}