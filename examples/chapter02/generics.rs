use std::fmt::Display;

// ジェネリック構造体
struct Pair<T: Display> {
    first: T,
    second: T,
}

// ジェネリック構造体にメソッドを実装
impl<T: Display> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }
}
// Displayを実装していない構造体
struct Point {
    x: i32,
    y: i32,
}

// impl Display for Point {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Point({}, {})", self.x, self.y)
//     }
// }

fn main() {
    let pair_of_ints = Pair::new(1, 2);
    let pair_of_floats = Pair::new(1.5, 2.5);
    println!(
        "pair_of_ints : x = {}, y = {}",
        pair_of_ints.first, pair_of_ints.second
    );
    println!(
        "pair_of_floats : x = {}, y = {}",
        pair_of_floats.first, pair_of_floats.second
    );

    // エラー: Pointは`Display`トレイトを実装していない
    //let pair_of_points = Pair::new(Point { x: 1, y: 2 }, Point { x: 3, y: 4 });
}
