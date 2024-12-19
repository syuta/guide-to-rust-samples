trait Animal {
    fn make_sound(&self) -> String;
}
struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "わんわん!".to_string()
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        "にゃー!".to_string()
    }
}

// 静的ディスパッチ
fn static_make_sound(animal: &impl Animal) {
    println!("{}", animal.make_sound());
}

// 動的ディスパッチ
fn dynamic_make_sound(animal: &dyn Animal) {
    println!("{}", animal.make_sound());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    // 静的ディスパッチ
    static_make_sound(&dog); // コンパイル時に Dog 用のコードが生成
    static_make_sound(&cat); // コンパイル時に Cat 用のコードが生成

    // 動的ディスパッチ
    dynamic_make_sound(&dog); // 実行時に vtable を参照
    dynamic_make_sound(&cat); // 実行時に vtable を参照
}
