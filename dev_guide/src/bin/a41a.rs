use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

struct Counter(usize);

fn main() {
    let counter: Counter = Counter(0);
    let shared_counter = Arc::new(Mutex::new(counter));

    let thread_1_counter = Arc::clone(&shared_counter);
    let thread_2_counter = shared_counter.clone();

    let thread_1 = thread::spawn(move || {
        let mut counter = thread_1_counter.lock();
        counter.0 += 1;
    });

    let thread_2 = thread::spawn(move || {
        let mut counter = thread_2_counter.lock();
        counter.0 += 1;
    });

    thread_1.join().and_then(|_| thread_2.join()).unwrap();
    println!("{}", shared_counter.lock().0);
}
