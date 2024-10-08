#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Copy for Rectangle {}

fn main() {
    // Point構造体の使用
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;  // p1がコピーされる
    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);

    // Rectangle構造体の使用
    let r1 = Rectangle { width: 30, height: 40 };
    let r2 = r1;  // r1がコピーされる
    println!("r1: {}x{}", r1.width, r1.height);
    println!("r2: {}x{}", r2.width, r2.height);

    // 関数に渡す例
    print_point(p1);
    print_rectangle(r1);

    // コピー後も元の変数が使用可能
    println!("Original p1 still accessible: ({}, {})", p1.x, p1.y);
    println!("Original r1 still accessible: {}x{}", r1.width, r1.height);
}

fn print_point(p: Point) {
    println!("Point: ({}, {})", p.x, p.y);
}

fn print_rectangle(r: Rectangle) {
    println!("Rectangle: {}x{}", r.width, r.height);
}