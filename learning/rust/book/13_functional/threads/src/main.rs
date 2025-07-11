use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}, {:?}", thread::current().id());

    thread::spawn(move || println!("From thread: {list:?}, {:?}", thread::current().id()))
        .join()
        .unwrap();
}
