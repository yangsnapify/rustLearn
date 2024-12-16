fn process_data<F>(data: i32, pipeline: Vec<Box<dyn Fn(i32) -> i32>>) -> i32 {
    let mut result = data;
    for step in pipeline {
        result = step(result);  // Apply each step in the pipeline
    }
    result
}

fn main() {
    let add_one = |x| x + 1;
    let multiply_by_three = |x| x * 3;

    let pipeline: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(add_one),
        Box::new(multiply_by_three),
    ];

    let result = process_data(5, pipeline);
    println!("Processed result: {}", result); // Output: 18 (5 + 1 = 6, 6 * 3 = 18)
}


fn compose<F, G, T>(f: F, g: G) -> Box<dyn Fn(T) -> T>
where
    F: Fn(T) -> T,
    G: Fn(T) -> T,
{
    Box::new(move |x: T| f(g(x))) // Composes f and g, applying g first, then f
}

fn main() {
    let add_one = |x: i32| x + 1;
    let multiply_by_two = |x: i32| x * 2;

    // Here, Rust infers that T is i32 based on the closures' signatures
    let composed = compose(add_one, multiply_by_two);
    println!("{}", composed(5));  // Output: 11 (5 * 2 + 1)
}


//Fat Pointers:
// Trait objects (&dyn Trait): A fat pointer that stores both a pointer to the data and a pointer to a vtable for dynamic dispatch.
// Slices (&[T]): A fat pointer that stores both a pointer to the data and the length of the slice.

// //smart pointers
// Box<T>: Allocates data on the heap with ownership and deallocation when it goes out of scope.
// Rc<T>: Enables shared ownership with reference counting (non-thread-safe).
// Arc<T>: A thread-safe version of Rc<T>.
// RefCell<T>: Provides interior mutability with runtime borrow checking (mutable access to an immutable structure).
// Cell<T>: Provides interior mutability but doesn't allow borrowing references.
// UnsafeCell<T>: Allows for unsafe interior mutability, typically used in low-level implementations.
// Pin<T>: Ensures that the data is not moved in memory, useful for async programming or self-referential structures.