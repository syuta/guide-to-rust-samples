// ジェネリック構造体
struct Pair<T> {
    first: T,
    second: T,
}

// ジェネリック構造体にメソッドを実装
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }
}

fn main() {
    let pair_of_ints = Pair::new(1, 2);
    let pair_of_floats = Pair::new(1.5, 2.5);
    println!("pair_of_ints : x = {}, y = {}",pair_of_ints.first,pair_of_ints.second);
    println!("pair_of_floats : x = {}, y = {}",pair_of_floats.first,pair_of_floats.second);
}
