// implブロックでのSelfの使用
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Selfは Point と同じ意味
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

// トレイト内でのSelfの使用
trait Shape {
    // Selfは、このトレイトを実装する型
    fn new() -> Self;
}

// Pointに対してShapeトレイトを実装
impl Shape for Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    // Point::new()を使用
    let p1 = Point::new();
    println!("Point: ({}, {})", p1.x, p1.y);

    // Shape::new()を使用
    let p2: Point = Shape::new();
    println!("Shape: ({}, {})", p2.x, p2.y);
}
