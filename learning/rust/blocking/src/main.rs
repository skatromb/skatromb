use futures::future::join_all;
use std::{thread, time};

fn sleep_ms(start: &time::Instant, id: u64, duration_ms: u64) {
    thread::sleep(time::Duration::from_millis(duration_ms));
    println!(
        "future {id} slept for {duration_ms}ms, finished after {}ms",
        start.elapsed().as_millis()
    );
}

#[tokio::main]
async fn main() {
    let start = time::Instant::now();
    let sleep_handles: Vec<tokio::task::JoinHandle<()>> = (1..=10).map(|t|
        tokio::task::spawn_blocking(move || {
            sleep_ms(&start.clone(), t, t * 10)
        })
    ).collect();
    join_all(sleep_handles).await;
}
