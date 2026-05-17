// ============================================================
// CONCEPT: Channels — Message Passing Between Threads
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# has: ConcurrentQueue<T>, Channel<T>, BlockingCollection<T>.
// Rust has: std::sync::mpsc (multi-producer, single-consumer channels).
// The pattern: "Do not communicate by sharing memory;
//               instead, share memory by communicating."
//
// mpsc = Multiple Producer, Single Consumer
//
// RUN: cargo run --bin channels
// ============================================================

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    basic_channel();
    multiple_producers();
    producer_consumer();
    channel_select_pattern();
}

fn basic_channel() {
    println!("=== Basic mpsc Channel ===");

    // mpsc::channel() creates a (sender, receiver) pair.
    // C# analogy: Channel<T>.CreateUnbounded() → (writer, reader)
    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || {
        // tx is MOVED into the thread
        tx.send("hello".to_string()).unwrap();
        tx.send("from".to_string()).unwrap();
        tx.send("thread".to_string()).unwrap();
        // tx dropped here — receiver knows no more messages coming
    });

    // recv() blocks until a message arrives or sender is dropped (Err)
    // C# analogy: await reader.ReadAsync()
    while let Ok(msg) = rx.recv() {
        println!("received: {msg}");
    }
    println!("channel closed");

    handle.join().unwrap();
}

fn multiple_producers() {
    println!("\n=== Multiple Producers ===");

    let (tx, rx) = mpsc::channel::<(usize, i32)>();

    // Clone the sender for each producer:
    // C# analogy: multiple tasks writing to the same Channel<T>
    let mut handles = vec![];
    for producer_id in 0..3 {
        let tx_clone = tx.clone();
        let h = thread::spawn(move || {
            for i in 0..3 {
                let value = (producer_id * 10 + i) as i32;
                tx_clone.send((producer_id, value)).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
            // tx_clone dropped when the thread ends
        });
        handles.push(h);
    }
    // Drop the original tx — otherwise receiver waits forever
    drop(tx);

    // Collect all messages:
    let mut messages: Vec<(usize, i32)> = rx.iter().collect();
    messages.sort();
    println!("received {} messages: {messages:?}", messages.len());

    for h in handles { h.join().unwrap(); }
}

fn producer_consumer() {
    println!("\n=== Producer-Consumer Pattern ===");

    // Work items sent from main thread, results collected back
    let (work_tx, work_rx) = mpsc::channel::<i32>();
    let (result_tx, result_rx) = mpsc::channel::<(i32, i32)>();

    // Spawn workers
    let mut workers: Vec<std::thread::JoinHandle<()>> = vec![];
    for worker_id in 0..3 {
        let work_rx = {
            // We need to share work_rx across workers.
            // For multiple consumers, use Arc<Mutex<Receiver>>:
            use std::sync::{Arc, Mutex};
            // We'll do it a different way: one worker per channel
            // (the standard mpsc pattern). For M:N, use a work queue.
            let _ = worker_id;
            None::<Arc<Mutex<mpsc::Receiver<i32>>>>
        };
        let _ = work_rx;
    }
    drop(workers); // clear for simplicity

    // Single worker:
    let result_tx2 = result_tx.clone();
    let h = thread::spawn(move || {
        for item in work_rx {
            let result = item * item; // "process" the item
            result_tx2.send((item, result)).unwrap();
        }
    });
    drop(result_tx); // drop original so receiver can know when done

    // Send work:
    for i in 1..=5 { work_tx.send(i).unwrap(); }
    drop(work_tx); // signal no more work

    // Collect results:
    for (input, output) in result_rx {
        println!("  {}² = {}", input, output);
    }

    h.join().unwrap();
}

fn channel_select_pattern() {
    println!("\n=== try_recv and Non-Blocking Receive ===");

    let (tx, rx) = mpsc::channel::<i32>();

    // Send some messages:
    tx.send(1).unwrap();
    tx.send(2).unwrap();
    tx.send(3).unwrap();

    // Non-blocking receive — like C# TryRead or TryDequeue:
    loop {
        match rx.try_recv() {
            Ok(val)                              => println!("  try_recv: {val}"),
            Err(mpsc::TryRecvError::Empty)       => { println!("  no more messages"); break; }
            Err(mpsc::TryRecvError::Disconnected)=> { println!("  sender disconnected"); break; }
        }
    }

    // recv_timeout — blocking with timeout:
    let (tx2, rx2) = mpsc::channel::<&str>();
    let h = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        tx2.send("late message").unwrap();
    });

    match rx2.recv_timeout(Duration::from_millis(2)) {
        Ok(msg)  => println!("received in time: {msg}"),
        Err(mpsc::RecvTimeoutError::Timeout)       => println!("timed out waiting"),
        Err(mpsc::RecvTimeoutError::Disconnected)  => println!("disconnected"),
    }

    h.join().unwrap();
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn channel_sends_and_receives() {
        let (tx, rx) = mpsc::channel::<i32>();
        thread::spawn(move || {
            tx.send(42).unwrap();
        }).join().unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn multiple_senders() {
        let (tx, rx) = mpsc::channel::<i32>();
        let handles: Vec<_> = (0..5).map(|i| {
            let tx = tx.clone();
            thread::spawn(move || tx.send(i).unwrap())
        }).collect();
        drop(tx);
        for h in handles { h.join().unwrap(); }
        let mut msgs: Vec<i32> = rx.iter().collect();
        msgs.sort();
        assert_eq!(msgs, vec![0, 1, 2, 3, 4]);
    }
}
