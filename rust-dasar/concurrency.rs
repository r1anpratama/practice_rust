use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread spawned count: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..3 {
        println!("Main thread count: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();
}