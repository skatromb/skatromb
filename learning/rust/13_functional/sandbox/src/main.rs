use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}
