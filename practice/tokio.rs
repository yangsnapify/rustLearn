
// What is tokio, and why is it used in Rust?
// tokio is a async runtime to rust

// How does tokio enable asynchronous programming in Rust?
// tokio enable async by providing a async runtime
#[tokio::main]

// How do async and await work with tokio?
#[tokio::main]
async fn main() {
    let result = do_work().await;
    println!("Result: {}", result);
}

// Can you explain the difference between blocking and non-blocking functions in Rust?
// block fn is when you writing sync fn, non block is async

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

async fn read_file_async() -> io::Result<String> {
    let mut file = File::open("some_file.txt").await?;
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

#[tokio::main]
async fn main() {
    match read_file_async().await {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}


// How does tokio::spawn() work, and why is it used in an async context?
// tokion spawn is provided by tokio runtime to exectue async task concurrently


// What does tokio::spawn() do, and what type of value does it return?
// tokio::spawn return a join handler, basically it wait for the task to finished, check for error and return the result


// Why would you use tokio::spawn() instead of just calling a function directly?
// becuase of async requirement

// What is join_all() in futures and how does it work?
// join_all is like promise all in js, it waits for multiple async future task to be done

// What does futures::join_all() do when passed a collection of futures?
// first it executed task concurrently and wait for all futures to be done then return the result


// Can you give an example where you spawn several tasks and use join_all() to await their completion?
use futures::future::join_all;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let futures = vec![
        async_task(1, 2),
        async_task(2, 1),
        async_task(3, 3),
    ];

    let results = join_all(futures).await;
    println!("Results: {:?}", results);
}

async fn async_task(id: u32, delay_secs: u64) -> u32 {
    println!("Task {} started", id);
    sleep(Duration::from_secs(delay_secs)).await;
    println!("Task {} finished", id);
    id * 10
}


// What is the advantage of using Box<dyn Fn> with Arc<Mutex<T>> in a multi-threaded application?
// box dyn provides flexibility because the exact type of the function doesn’t need to be known at compile time
// arc mutex is meant to design to shared data acroos multiple threads so mutex allow you to safely handle data and arc allo you to share data across threads