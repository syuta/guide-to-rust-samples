fn main() {
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}", s1); // コンパイルエラー
        println!("{}", s2);
    } //ここでドロップされる
}
