trait Container<T = String> {
    fn get_value(&self) -> &T;
    fn set_value(&mut self, value: T);
}

// デフォルトの型パラメータを持つ構造体
struct MyBox<T = i32> {
    value: T,
}

// Box構造体の実装
impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox { value }
    }
}

// Container トレイトの実装
impl<T> Container<T> for MyBox<T> {
    fn get_value(&self) -> &T {
        &self.value
    }

    fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

fn main() {
    // デフォルトの型パラメータ(i32)を使用
    let default_box = MyBox::new(42);
    println!("Default box value: {}", default_box.get_value());

    // 明示的に型を指定
    let mut string_box: MyBox<String> = MyBox::new(String::from("Hello"));
    println!("String box value: {}", string_box.get_value());

    // 値を変更
    string_box.set_value(String::from("World"));
    println!("Updated string box value: {}", string_box.get_value());
}
