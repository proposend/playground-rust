use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    {
        // The API of Mutex<T>
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }


    {
        // Sharing a Mutex<T> Between Multiple Threads
        // let counter = Mutex::new(0);

        // Multiple Ownership with Multiple Threads
        // let counter = Rc::new(Mutex::new(0));

        // Atomic Reference Counting with Arc<T>
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
    {
        // Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>
        // Rc<T> values refer to each other, causing memory leaks
        // Mutex<T> comes with the risk of creating deadlocks
    }
}
