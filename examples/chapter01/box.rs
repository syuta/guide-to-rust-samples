fn main() {
    // Boxの作成
    let box_int = Box::new(123);
    println!("Boxed integer: {}", box_int);

    // 大きなデータ
    let large_data = Box::new([0; 1000]);
    println!("Large array length: {}", large_data.len());

    // 再帰的な構造体
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // リスト構造の作成
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("List: {:?}", list);

    // メモリリークをわざと作成
    let static_str: &'static str = Box::leak(Box::new(String::from("Hello Box")));
    println!("Leaked string: {}", static_str);

    // Boxのデリファレンス
    let boxed_num = Box::new(42);
    let unboxed_num = *boxed_num; // Boxの中身を取り出す
    println!("Unboxed number: {}", unboxed_num);

    // Box::into_raw()とBox::from_raw()
    let box_val = Box::new(100);
    let raw_ptr = Box::into_raw(box_val);
    unsafe {
        let box_back = Box::from_raw(raw_ptr);
        println!("Recovered value: {}", *box_back);
    }
}
