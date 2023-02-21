use std::{
    sync::{mpsc, Mutex, Arc},
    thread,
    time::Duration,
};

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx_clone = tx.clone();
    thread::spawn(move || {
        let values = vec!["th1 hi", "th1 from", "th1 the", "th1 thread"];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let values = vec!["th2 more", "th2 messages", "th2 for", "th2 you"];

        for val in values {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }

    //Shared-State Concurrency
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);


    let counter = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num *= 2;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
