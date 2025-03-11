use std::borrow::Cow;

fn process_string(input: &str) -> Cow<str> {
    if input.contains("foo") {
        // 変更が必要な場合のみ所有権のある値を作成
        Cow::Owned(input.replace("foo", "bar"))
    } else {
        // 変更が不要な場合は借用のまま
        Cow::Borrowed(input)
    }
}

fn main() {
    let s1 = "hello foo world";
    let s2 = "hello world";

    println!("{}", process_string(s1)); // "hello bar world"
    println!("{}", process_string(s2)); // "hello world"
}
