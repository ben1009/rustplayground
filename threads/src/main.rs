use std::fmt::Debug;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut handlers = vec![];
    let num = Arc::new(AtomicI32::new(0));
    // let (tx, rx) = mpsc::channel();

    for i in 1..10 {
        let num = Arc::clone(&num);
        let handler = thread::spawn(move || {
            num.fetch_add(1, Ordering::Relaxed);
            thread::sleep(Duration::from_micros(10));
            println!("{}", num.load(Ordering::SeqCst));
            // let mut count = lock.lock().unwrap();
            // *count += 1;
            // tx1.send(i).unwrap();
            // drop(tx1);
        });
        handlers.push(handler);
    }
    // drop(tx);
    for handler in handlers {
        handler.join().unwrap();
    }
}
