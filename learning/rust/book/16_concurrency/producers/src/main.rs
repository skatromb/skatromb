use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;


fn send_fn<T>(tx: Sender<T>, vals: Vec<T>) {
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1))
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let vals = vec![
        "hi".to_string(),
        "from".to_string(),
        "the".to_string(),
        "thread".to_string(),
    ];

    let vals1 = vec![
        "MORE".to_string(),
        "MESSAGES".to_string(),
        "FOR".to_string(),
        "YOU".to_string(),
    ];
    
    thread::spawn(move || send_fn(tx, vals));
    thread::spawn(move || send_fn(tx1, vals1));
    
    for received in rx {
        println!("Got: {received}");
    }
}
