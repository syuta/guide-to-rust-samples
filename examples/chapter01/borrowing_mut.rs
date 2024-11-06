fn main() {
    let mut s = String::from("hello mut borrowing");

    change(&mut s);
    println!("s is now: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
