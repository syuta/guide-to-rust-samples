use std::error::Error;
use std::fmt;

// シンプルな内部エラー
#[derive(Debug)]
struct DatabaseError(String);

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "データベースエラー: {}", self.0)
    }
}

impl Error for DatabaseError {}

// 外部エラー（内部エラーを包含）
#[derive(Debug)]
struct AppError {
    message: String,
    source: Box<dyn Error>,  // エラーの原因
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "アプリケーションエラー: {}", self.message)
    }
}

impl Error for AppError {
    // source メソッドをオーバーライドしてエラー原因を返す
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

// エラーを発生させる
fn fetch_data() -> Result<(), Box<dyn Error>> {
    // 内部エラーを作成
    let db_error = DatabaseError("接続タイムアウト".to_string());
    
    // 内部エラーを包含した外部エラーを返す
    let app_error = AppError {
        message: "データの取得に失敗しました".to_string(),
        source: Box::new(db_error),
    };
    
    Err(Box::new(app_error))
}

fn main() {
    if let Err(error) = fetch_data() {
        println!("Error: {}", error);
        
        // エラーチェーンを辿る
        let mut cause = error.source();
        while let Some(err) = cause {
            println!("cause: {}", err);
            cause = err.source();
        }
    }
}
