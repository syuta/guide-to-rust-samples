// トレイトでの関連定数
trait Animal {
    //足の数
    const LEGS: u8;
}

struct Dog;
struct Spider;

impl Animal for Dog {
    const LEGS: u8 = 4; // 犬の足の数
}

impl Animal for Spider {
    const LEGS: u8 = 8; // クモの足の数
}

// 構造体での関連定数の例
struct Circle {
    radius: f64,
}

impl Circle {
    const PI: f64 = 3.14159; // 円周率を関連定数として定義

    fn area(&self) -> f64 {
        Self::PI * self.radius * self.radius
    }
}

fn main() {
    // トレイトの関連定数を使用
    println!("Number of dog's legs: {}", Dog::LEGS);
    println!("Number of spider's legs: {}", Spider::LEGS);

    // 構造体の関連定数を使用
    let circle = Circle { radius: 2.0 };
    println!("PI value: {}", Circle::PI);
    println!("Circle area: {}", circle.area());
}
