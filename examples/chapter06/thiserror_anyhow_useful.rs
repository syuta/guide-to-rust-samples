use anyhow::{Context, Result, anyhow};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use thiserror::Error;
use std::fmt;

// ========== 透過的なエラー型 ==========
// 外部ライブラリのエラーを模したもの
#[derive(Debug)]
struct HttpError {
    status_code: u16,
    message: String,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP error {}: {}", self.status_code, self.message)
    }
}

impl std::error::Error for HttpError {}

#[derive(Debug)]
struct JsonError(String);

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "JSON error: {}", self.0)
    }
}

impl std::error::Error for JsonError {}

#[derive(Error, Debug)]
enum ApiError {
    #[error("HTTPリクエスト失敗: {status_code}")]
    HttpError {
        #[source]  // 元のエラーを「原因」として保持
        source: HttpError,
        status_code: u16,
    },
    
    #[error("JSONデコードエラー: {0}")]
    JsonError(#[from] JsonError),
    
    #[error("データ検証エラー: {0}")]
    ValidationError(String),
}

// ========== 結果型のエイリアス ==========
// 結果型のエイリアス定義
type ApiResult<T> = Result<T, ApiError>;

// ========== エラーの変換と拡張 ==========
// 外部ライブラリのエラー
#[derive(Debug)]
struct NetworkError(String);

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for NetworkError {}

// アプリのエラー
#[derive(Error, Debug)]
enum AppError {
    #[error("ネットワークエラー: {0}")]
    Network(String),
    
    #[error("I/Oエラー: {0}")]
    Io(#[from] std::io::Error),
}

// 外部エラーからの変換
impl From<NetworkError> for AppError {
    fn from(err: NetworkError) -> Self {
        AppError::Network(err.0)
    }
}

// ========== 関数実装 ==========

// エラーのコンテキストを追加する例
fn read_username() -> Result<String> {
    // ファイルを開く際にコンテキスト情報を追加
    let mut file = File::open("user.txt")
        .with_context(|| "ユーザーファイルを開けませんでした")?;
    
    // 内容を読み取る際にもコンテキスト情報を追加
    let mut name = String::new();
    file.read_to_string(&mut name)
        .with_context(|| "ユーザー名の読み取りに失敗しました")?;
    
    // 空のファイルの場合はエラー
    if name.trim().is_empty() {
        return Err(anyhow!("ユーザー名が設定されていません"));
    }
    
    Ok(name.trim().to_string())
}

// エラーチェーンとバックトレースの例
fn parse_config(content: &str) -> Result<i32> {
    content.trim().parse::<i32>()
        .context("設定値を整数として解析できませんでした")
}

fn load_config(path: &str) -> Result<i32> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("{}の読み取りに失敗", path))?;
    
    parse_config(&content)
        .context("設定の解析中にエラーが発生")
}

// エラーのダウンキャストの例
fn read_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("{}の読み取りに失敗", path.display()))
}

// 透過的なエラー型の例
fn fetch_data(url: &str) -> ApiResult<String> {
    // HTTPリクエストをシミュレート
    if !url.starts_with("https") {
        let http_error = HttpError {
            status_code: 400,
            message: "HTTPSのみサポートしています".to_string(),
        };
        
        return Err(ApiError::HttpError {
            source: http_error,
            status_code: 400,
        });
    }
    
    // JSONパースエラーをシミュレート
    if url.contains("invalid") {
        return Err(ApiError::JsonError(JsonError("不正なJSON形式".to_string())));
    }
    
    // 検証エラーをシミュレート
    if url.contains("empty") {
        return Err(ApiError::ValidationError("空のレスポンス".to_string()));
    }
    
    Ok("{ \"data\": \"dummy response\" }".to_string())
}

// 結果型エイリアスを使用した別の関数
fn validate_and_process(data: &str) -> ApiResult<i32> {
    if data.is_empty() {
        return Err(ApiError::ValidationError("データが空です".to_string()));
    }
    // 何らかの処理
    Ok(42)
}

fn process_api_data(url: &str) -> ApiResult<i32> {
    let data = fetch_data(url)?;
    validate_and_process(&data)
}

// エラーの変換と拡張の例
fn fetch_network_data() -> Result<(), NetworkError> {
    Err(NetworkError("接続タイムアウト".to_string()))
}

fn process_network_data() -> Result<(), AppError> {
    fetch_network_data()?;  // NetworkError → AppError::Network
    Ok(())
}

fn main() {
    // エラーのコンテキストを追加する例
    println!("===== エラーのコンテキストを追加する例 =====");
    match read_username() {
        Ok(name) => println!("こんにちは、{}さん", name),
        Err(e) => eprintln!("エラー: {:#}", e),
    }
    
    // エラーチェーンとバックトレースの例
    println!("\n===== エラーチェーンとバックトレースの例 =====");
    match load_config("config.txt") {
        Ok(value) => println!("設定値: {}", value),
        Err(e) => {
            // 簡潔な表示
            println!("エラー: {}", e);
            // 詳細なエラーチェーンの表示
            println!("詳細: {:#}", e);
            // 手動でエラーチェーンを辿る
            let mut source = e.source();
            while let Some(cause) = source {
                println!("原因: {}", cause);
                source = cause.source();
            }
        }
    }
    
    // エラーのダウンキャストの例
    println!("\n===== エラーのダウンキャストの例 =====");
    let path = Path::new("missing.txt");
    match read_file(path) {
        Ok(content) => println!("内容: {}", content),
        Err(e) => {
            // io::ErrorKind::NotFoundにダウンキャスト
            if let Some(io_err) = e.downcast_ref::<io::Error>() {
                if io_err.kind() == io::ErrorKind::NotFound {
                    println!("ファイルが見つかりません: {}", path.display());
                } else {
                    println!("I/Oエラー: {}", io_err);
                }
            } else {
                eprintln!("予期しないエラー: {}", e);
            }
        }
    }
    
    // 透過的なエラー型と結果型エイリアスの例
    println!("\n===== 透過的なエラー型と結果型エイリアスの例 =====");
    
    // 各種エラーケースをテスト
    let urls = [
        "http://example.com/api",   // HTTPSでないのでエラー
        "https://example.com/invalid", // JSONエラー
        "https://example.com/empty",   // 検証エラー
        "https://example.com/valid",   // 成功
    ];
    
    for url in &urls {
        println!("URLをテスト: {}", url);
        match process_api_data(url) {
            Ok(value) => println!("  API処理結果: {}", value),
            Err(e) => match e {
                ApiError::HttpError { status_code, .. } => {
                    println!("  HTTPエラー (ステータスコード: {})", status_code)
                },
                ApiError::JsonError(_) => println!("  JSONパースエラー"),
                ApiError::ValidationError(msg) => println!("  検証エラー: {}", msg),
            },
        }
    }
    
    // エラーの変換と拡張の例
    println!("\n===== エラーの変換と拡張の例 =====");
    match process_network_data() {
        Ok(()) => println!("ネットワーク処理成功"),
        Err(e) => println!("ネットワーク処理失敗: {}", e),
    }
}