// 構造体の定義
struct Person {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

fn main() {
    // 元となるインスタンスを作成
    let person1 = Person {
        name: String::from("taro"),
        age: 30,
        email: String::from("taro@example.com"),
        active: true,
    };

    // 構造体更新構文を使って新しいインスタンスを作成
    // nameとageだけを変更し、他のフィールドはperson1から継承
    let person2 = Person {
        name: String::from("hanako"),
        age: 25,
        ..person1 // 構造体更新構文
    };

    // 確認
    println!("person2.name: {}", person2.name); // hanako
    println!("person2.age: {}", person2.age); // 25
    println!("person2.email: {}", person2.email); // taro@example.com
    println!("person2.active: {}", person2.active); // true
}
