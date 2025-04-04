use std::fmt;
use std::error::Error;

// エラーを表す構造体
#[derive(Debug)]
struct MyError {
    message: String,
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {}

fn cause_error() -> Result<(), MyError> {
    Err(MyError { message: String::from("エラーが発生しました") })
}

fn main() {
    match cause_error() {
        Ok(()) => println!("成功"),
        Err(e) => println!("エラー: {}", e),
    }
}
