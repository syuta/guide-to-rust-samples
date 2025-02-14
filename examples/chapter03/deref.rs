use std::ops::{Deref, DerefMut};

// MyBoxの定義
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


// Derefの実装
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// DerefMutの実装
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {

    // 不変の例
    let x = MyBox::new(5);
    assert_eq!(5, *x);  // 参照外しで値にアクセス

    // 可変の例
    let mut y = MyBox(String::from("hello"));
    
    // 以下の3つは同じ動作をする
    // 1. 自動的なDeref強制変換を使用（最もシンプル）
    y.push_str(" world");
    
    // 2. 明示的なderef_mutを使う
    (*y).push_str(" world");
    
    // 3. すべての変換を明示的に記述
    let string_ref: &mut String = y.deref_mut();
    string_ref.push_str(" world");
    
    println!("{}", *y);  // "hello world world world"
}