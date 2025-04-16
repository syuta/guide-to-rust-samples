use std::fmt::Display;

// 基本的な図形トレイト - 表示可能
trait Shape: Display {
    fn area(&self) -> f64;
}

// 円の実装
struct Circle {
    radius: f64,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle with radius {}", self.radius)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Displayトレイトを使う関数
fn print_info(item: &dyn Display) {
    println!("Information: {}", item);
}

// Shapeトレイトを使う関数
fn print_shape_details(shape: &dyn Shape) {
    println!("Shape: {}", shape);
    println!("Area: {:.2} square units", shape.area());
    
    // Rust 1.86からは、&dyn Shape を &dyn Display に直接アップキャストできる
    print_info(shape);
}

fn main() {
    let circle = Circle { radius: 5.0 };
    
    // Circleから&dyn Shapeを作成
    let shape: &dyn Shape = &circle;
    
    // 図形の詳細を表示
    print_shape_details(shape);
    
    // Rust 1.86からは、&dyn Shape を &dyn Display に直接渡せる
    print_info(shape);
}
