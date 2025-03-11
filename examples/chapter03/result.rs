use std::fmt;
use std::fs::File;
use std::io::{self, Read};

// カスタムエラー型の定義
#[derive(Debug)]
struct CustomError {
    message: String,
}

impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError {
            message: msg.to_string(),
        }
    }
}

// エラーメッセージの表示用にDisplayトレイトを実装
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

// std::error::Errorトレイトを実装
impl std::error::Error for CustomError {}

// ?演算子を使用する関数の例
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    // 基本的な使い方
    let file_result = File::open("non_existent.txt");

    // matchでの使用（参照を使用）
    match &file_result {
        Ok(_file) => println!("ファイルを開きました"),
        Err(error) => println!("エラー: {}", error),
    }

    // if letでの使用（別の変数として扱う）
    let another_result = File::open("non_existent.txt");
    if let Ok(_file) = another_result {
        println!("ファイルを開きました");
    }

    // Result with unwrap関連
    let result: Result<i32, &str> = Ok(10);

    // unwrap - エラーの場合はパニック
    let value = result.unwrap(); // 10
    println!("unwrap: {}", value);

    // unwrap_or - エラーの場合はデフォルト値を返す
    let value = result.unwrap_or(0); // 10
    println!("unwrap_or: {}", value);

    let err_value: Result<i32, &str> = Err("error");
    let value = err_value.unwrap_or(0); // 0
    println!("error unwrap_or: {}", value);

    // expect - カスタムメッセージでパニック
    let value = result.expect("Failed to get value");
    println!("expect: {}", value);

    // ?演算子の使用例
    match read_file("example.txt") {
        Ok(contents) => println!("ファイルの内容: {}", contents),
        Err(e) => println!("ファイルの読み込みに失敗: {}", e),
    }

    // map - 成功値を変換
    let result: Result<i32, &str> = Ok(10);
    let doubled = result.map(|x| x * 2);
    println!("map: {:?}", doubled);

    // map_err - エラーの型を変換
    let result: Result<i32, &str> = Err("error occurred");
    let converted = result.map_err(CustomError::new);
    println!("map_err: {:?}", converted);

    // and_then の例 (Result版)
    let result: Result<i32, &str> = Ok(5);
    let chained = result.and_then(|n| {
        if n > 0 {
            Ok(n * 2)
        } else {
            Err("number must be positive")
        }
    });
    println!("and_then: {:?}", chained);

    // or_else の例 (Result版)
    let error_result: Result<i32, &str> = Err("original error");
    let recovered: Result<i32, &str> = error_result.or_else(|_| Ok(42));
    println!("or_else: {:?}", recovered);
}
