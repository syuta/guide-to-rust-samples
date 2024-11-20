#[derive(Clone)]
struct Square {
    side_length: i32,
}

fn main() {
    let original = Square { side_length: 5 };
    let cloned = original.clone();
    println!("Original square side length: {}", original.side_length);
    println!("Clone square side length: {}", cloned.side_length);
}
