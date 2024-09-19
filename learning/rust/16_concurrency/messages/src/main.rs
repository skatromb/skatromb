use std::sync::mpsc;
use std::{thread, time};

fn main() {
    let (tx, rx) = mpsc::channel();
    let _tx1 = tx.clone();

    thread::spawn(move || {
        let val = "hi".to_string();
        tx.send(val).unwrap();
    });

    let received: String;
    let timer = time::Instant::now();
    loop {
        println!("Trying to get the message");

        if let Ok(rec) = rx.try_recv() {
            received = rec;
            break
        }
    }

    println!("Received!: {received} in {:?}", timer.elapsed());
}
