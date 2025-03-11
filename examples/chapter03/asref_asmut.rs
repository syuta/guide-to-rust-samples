// AsRefを使用した関数
fn process_str<T: AsRef<str>>(arg: T) {
    println!("arg: {}", arg.as_ref());
}

// AsMutを使用した関数
fn process_slice_mut<T: AsMut<[i32]> + std::fmt::Debug>(m_arg: &mut T) {
    let slice = m_arg.as_mut();
    slice[0] = 100;
    println!("m_arg: {:?}", m_arg);
}

fn main() {
    // AsRefの使用例
    let string = String::from("hello");
    process_str(&string); // Stringから&strへの変換

    // AsMutの使用例
    let mut vec = vec![1, 2, 3];
    process_slice_mut(&mut vec); // Vec<i32>から&mut [i32]への変換
}
