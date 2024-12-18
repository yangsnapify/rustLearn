// What is Arc in Rust, and why do we need it for concurrency?
// arc is a smart pointer allows multiple threads can safely share data

// sync primitive types
    Mutex: For exclusive access to mutable data.
    RwLock: For shared access to data with readers and writers.
    AtomicUsize, AtomicBool : For lock-free operations on simple types ((fetch_add, compare_and_swap, load, store) ).
    Condvar: For coordinating threads based on conditions.


// How does Arc enable shared ownership of data across multiple threads?
// answer: its becuase of reference counting inside arc, basically when using arc we can have multiple threads to access but only one can write at the same time
// second thing is it use atomic operations to ensure safety, when you clone an arc the internal reference counts update

// What is the difference between Rc and Arc?
// the different betwwen rc and arc is arc using aromic operations. because arc is designed for working with multiple threads so it reuqire using atomic operations
// and rc doesnt


// When would you use Arc in a multi-threaded environment?
// answer: arc is designed for multi threaded therefore, use arc in multithreaded situation where you need to shared data across multple threads

// What is Mutex in Rust, and how does it work with Arc?
// answer: mutex provide exclusive ownership while arc provide shared ownership across multiple threads


// How does Mutex enable safe mutable access to data in a concurrent environment?
//  Locking and Unlocking Mechanism

// What is the role of Mutex::lock() in preventing data races?
// When a thread calls Mutex::lock(), it attempts to acquire the mutex. If the mutex is unlocked 
// (i.e., not currently held by any other thread), 
// the calling thread gains exclusive ownership of the mutex and can proceed to access the shared resource.
// Exclusive ownership means that while a thread holds the mutex, no other thread can access the shared resource protected by that mutex. 
// This prevents multiple threads from modifying or reading the shared data simultaneously.

// What happens if you try to lock a Mutex that is already locked? How can you handle that situation?
// if you try to lock a mutex that already locked the calling thread will be block until the lock become available

// What does Mutex::lock() do in Rust?
// asnwer: mutex lock is called for first acquiring the lock, if the lock is in use the lock cant be acquired therefore you have been block
// and you need to wait till the lock becomes available(or saying the mutexgurad dropped it automatic release the lock)

// What is the return type of Mutex::lock()?
// asnwer: its mutexguard and it contains a reference to the data

// How does it help avoid data races in concurrent programming?
// asnwer: We typically use exclusive access types, like Mutex<T>, RwLock<T>, Atomic<T>, and Box<T>, to handle data in concurrent programming. These types help avoid data races by ensuring that only one thread can access or modify the data at a time, either through mutual exclusion (locks) or atomic operations, 
//thereby preventing multiple threads from modifying the same data simultaneously.


// How do you handle the possibility of Mutex::lock() failing?
// failing means mutex is poinsoned, a mutex is poisoned becuase of threads encounter panic while locking
// we do a try catch at mutex lock


// What is iter_mut() and how does it work with Vec in Rust?
// its a interavle that provide mutable reference &mut Vec<T>
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.iter_mut() {
        *num *= 2;
    }

    println!("{:?}", numbers); // Output: [2, 4, 6, 8, 10]
}



// What is the purpose of iter_mut()?
// asnwer: to allow mutable iterable, is more efficient in some cases

// How is iter_mut() different from iter()?
// answer: the main diff is &t vs &mut t


// Can you use iter_mut() to modify elements in a shared, locked Vec?
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Initialize the Arc and Mutex
    let d1 = Arc::new(Mutex::new(vec![1, 2, 3, 4])); 

    // Clone the Arc to share ownership across threads
    let clone_arc = Arc::clone(&d1);

    let handle = thread::spawn(move || {
        let mut locked_vec = clone_arc.lock().unwrap();

        for num in locked_vec.iter_mut() {
            *num += 1;
        }
    });

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    // Access the updated vector in the main thread
    let locked_vec = d1.lock().unwrap();
    println!("{:?}", *locked_vec);
}


// write a map function
fn map<T, F>(v: &mut Vec<T>, f: F) -> Vec<T>
where F: FnMut(&mut T)
{
    let mut result = Vec::new();
    for element in v.iter_mut() {
        f(element);
        result.push(*element);
    }
    result
}


// ordering::SeqCst


// cell/ refcell
// cell provide interior mutability for types that implement copy trait, it copy the value in and out so its not a reference to the value
// RefCell provides interior mutability for any type, not just types that implement the Copy trait. It uses runtime borrow checking to allow mutable or immutable references to its value, ensuring borrowing rules are followed.




// Box<T> => fat pointers, 
// Rc<T>, Arc<T>, RefCell<T>, or Cell<T>. => smart pointers
// mutex, rwlock => sync primitives types



You need shared ownership (e.g., Rc<T>, Arc<T>).
You need exclusive ownership (Box<T>,)
You need exclusive access (e.g. Mutex<T> RwLock<T>, AtomicBool, AtomicUsize, AtomicIsize).
You need to allow interior mutability (e.g., Cell<T>, RefCell<T>).
Box<T>: Ensures exclusive ownership of the data (but not thread-safe for shared access). Used for single-owner data that does not need to be shared between threads.
Mutex<T>: Provides exclusive access to data via locking. Only one thread can access the data at a time. Suitable for shared mutable access across threads.
RwLock<T>: Allows multiple readers and one writer. Suitable for shared read access, with exclusive access for writes.
Atomic types (AtomicBool, AtomicUsize, AtomicIsize, etc.): Allow lock-free atomic operations on shared data, useful for low-level, simple operations where performance and atomicity are crucial.