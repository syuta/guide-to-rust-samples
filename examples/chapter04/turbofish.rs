use std::collections::HashMap;

// 浮動小数点から整数への変換
fn convert<T, U>(value: T) -> U
where
    T: std::fmt::Display,
    U: std::str::FromStr,
    U::Err: std::fmt::Debug,
{
    // 小数点数から整数への変換の場合、小数点以下を切り捨て
    let s = value.to_string();
    if let Some(pos) = s.find('.') {
        s[..pos]
            .parse::<U>()
            .unwrap_or_else(|_| "0".parse::<U>().unwrap())
    } else {
        // 小数点がない場合はそのままパース
        s.parse::<U>().unwrap()
    }
}

fn main() {
    println!("=== 文字列のパース ===");
    // ターボフィッシュを使用して明示的にi32に変換
    let num = "45".parse::<i32>().unwrap();
    println!("パースした数値: {}", num);

    println!("\n=== コレクションの初期化 ===");
    // 空のベクターを作成する際に型を明示
    let empty_vec = Vec::<i32>::new();
    println!("空のベクター: {:?}", empty_vec);

    // ハッシュマップの型を明示
    let mut scores = HashMap::<String, i32>::new();
    scores.insert(String::from("Alice"), 100);
    println!("スコアマップ: {:?}", scores);

    // collect() でコレクションに変換する際の型指定
    let chars1: Vec<char> = "hello".chars().collect();
    // ターボフィッシュを使うパターン
    let chars2 = "world".chars().collect::<Vec<char>>();
    println!("文字ベクター1: {:?}", chars1);
    println!("文字ベクター2: {:?}", chars2);

    println!("\n=== ジェネリック関数の型指定 ===");
    // 戻り値の型だけを指定
    let int_value = convert::<_, i32>(3.14);
    println!("変換結果 (f64 -> i32): {}", int_value); // 3

    // 両方の型パラメータを指定
    let float_value = convert::<i32, f64>(42);
    println!("変換結果 (i32 -> f64): {}", float_value); // 42.0

    // 型推論と明示的指定の比較
    let inferred: u8 = convert(120_i32);
    let explicit = convert::<i32, u8>(120);
    println!("推論による変換: {}, 明示的変換: {}", inferred, explicit);
}
