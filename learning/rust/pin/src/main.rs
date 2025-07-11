use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::{Duration, sleep};

// A work item. In this case, just sleep for the given time and respond
// with a message on the `respond_on` channel.
#[derive(Debug)]
struct Work {
    input: u32,
    sender: oneshot::Sender<u32>,
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await;
        println!("work result for iteration {i}: {resp}");
    }
}

// A worker which listens for work on a queue and performs it.
async fn worker(mut work_queue: mpsc::Receiver<Work>) {
    let mut _iterations = 0;
    let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));

    loop {
        tokio::select! {
            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await; // Pretend to work.
                work.sender
                    .send(work.input * 1000)
                    .expect("failed to send response");
                _iterations += 1;
            }
            _ = &mut timeout_fut => {
                println!("Timeout happened"); 
                timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
            },
        }
    }
}

// A requester which requests work and waits for it to complete.
async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    work_queue
        .send(Work { input, sender: tx })
        .await
        .expect("failed to send on work queue");
    rx.await.expect("failed waiting for response")
}
