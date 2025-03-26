use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// Atomicを使用した静的変数
static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    println!("initial value: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));
    
    // 複数スレッドからカウンター増加
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            for _ in 0..1000 {
                // スレッドセーフ
                ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    // すべてのスレッドの終了を待つ
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("final value: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));
}
