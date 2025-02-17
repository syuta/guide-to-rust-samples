use std::cell::UnsafeCell;

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    // TがCopyなら値を安全に取得可能
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // unsafeブロック内でポインタから値を読む
        unsafe { *self.value.get() }
    }

    // 値を更新
    pub fn set(&self, new: T) {
        // unsafeブロック内でポインタを使い、内部の値を書き換え
        unsafe {
            *self.value.get() = new;
        }
    }
}

fn main() {
    let cell = MyCell::new(42);
    println!("Initial value: {}", cell.get());
    cell.set(100);
    println!("Updated value: {}", cell.get());
}
