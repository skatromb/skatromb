use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let n_threads = thread::available_parallelism().unwrap().get();

    let handles: Vec<_> =
        (0..n_threads).map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            })
        }).collect();

    handles.into_iter().for_each(|handle|
        handle.join().unwrap()
    );

    println!("Result: {}", *counter.lock().unwrap());
}
