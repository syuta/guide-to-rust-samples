// ジェネリクスを使用したトレイト
trait Container<T> {
    fn get_value(&self) -> &T;
    fn set_value(&mut self, value: T);
}

// 格納用の構造体
struct Storage {
    name: String,
}

struct Storage2 {
    name: String,
}

// 同じStorage型に対して、異なる型パラメータで複数の実装が可能
impl Container<String> for Storage {
    fn get_value(&self) -> &String {
        &self.name
    }

    fn set_value(&mut self, value: String) {
        self.name = value;
    }
}

impl Container<i32> for Storage {
    fn get_value(&self) -> &i32 {
        &42 // 簡略化のため固定値
    }

    fn set_value(&mut self, _value: i32) {
        // 実装例
    }
}

// 一方、関連型では以下のような複数実装はコンパイルエラーとなる
trait ContainerAssoc {
    type Item;
    fn get_value(&self) -> &Self::Item;
    fn set_value(&mut self, value: Self::Item);
}

impl ContainerAssoc for Storage2 {
    type Item = String;
    fn get_value(&self) -> &String {
        &self.name
    }

    fn set_value(&mut self, value: String) {
        self.name = value;
    }
}
/*
// コンパイルエラー！
impl ContainerAssoc for Storage2 {
    type Item = i32;  // エラー：既にStorage2に対する実装が存在する
    // ...実装...
}
*/

fn main() {
    //ジェネリクスを使用
    let mut storage = Storage {
        name: "test".to_string(),
    };
    // 関連型を使用
    let mut storage2 = Storage2 {
        name: "test".to_string(),
    };

    // String型として使用
    Container::<String>::set_value(&mut storage, "Hello".to_string());
    println!("String value: {}", Container::<String>::get_value(&storage));

    // i32型として使用
    Container::<i32>::set_value(&mut storage, 42);
    println!("Integer value: {}", Container::<i32>::get_value(&storage));

    // 関連型を使用
    ContainerAssoc::set_value(&mut storage2, "Hello2".to_string());
    print!("String value: {}", storage2.get_value());
}
