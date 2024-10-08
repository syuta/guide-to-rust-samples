fn main() {
    let s = String::from("hello function");
    take_ownership(s);

    // println!("{}", s); // コンパイルエラー
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // some_stringがドロップされる
