use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;

// カスタムエラー型
#[derive(Debug)]
struct AppError(String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AppError {}

// 異なる種類のエラーを返す可能性のある関数
fn process_data(filename: &str) -> Result<i32, Box<dyn Error>> {
    // ファイルを開く - io::Error の可能性
    let mut file = File::open(filename)?;
    
    // 内容を読み込む - io::Error の可能性
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    // 数値に変換 - ParseIntError の可能性
    let number: i32 = content.trim().parse()?;
    
    // カスタム検証 - AppError の可能性
    if number < 0 {
        return Err(Box::new(AppError("Negative numbers are not allowed".to_string())));
    }
    
    Ok(number * 2)
}

fn main() {
    match process_data("number.txt") {
        Ok(result) => println!("reslt: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
