use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // === Cell ===
    println!("\n# Cell Example");
    let cell = Cell::new(1);
    cell.set(2);
    println!("Cell value: {}", cell.get());

    // === RefCell ===
    println!("\n# RefCell Example");
    let ref_cell = RefCell::new(vec![1, 2, 3]);
    ref_cell.borrow_mut().push(4);
    println!("RefCell value: {:?}", ref_cell.borrow());

    // === Mutex ===
    println!("\n# Mutex Example");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
