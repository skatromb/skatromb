use std::thread;
use std::time::{Instant, Duration};

fn main() {
    dbg!(thread::available_parallelism().unwrap());
    let timer = Instant::now();
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread {:?}!", thread::current().id());
            thread::sleep(Duration::from_millis(1));
        }
    });
    println!("{}", timer.elapsed().as_nanos());
    
    for i in 1..5 {
        println!("hi number {i} from main thread {:?}!", thread::current().id());
        thread::sleep(Duration::from_millis(1));
    }

    while !dbg!(handle.is_finished()) {
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
