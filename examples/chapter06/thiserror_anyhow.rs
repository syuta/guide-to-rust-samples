use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;
use anyhow::{Context, Result};

// thiserrorでエラー型定義
#[derive(Error, Debug)]
enum ConfigError {
    #[error("ファイルの読み込み中にエラーが発生しました")]
    IoError(#[from] io::Error),
    
    #[error("設定の形式が無効です: {0}")]
    InvalidFormat(String),
}

// 独自のエラー型を返す(ライブラリ関数と仮定)
fn lib_parse_config(path: &Path) -> std::result::Result<u32, ConfigError> {
    // ファイルを読み込む（IoErrorが発生する可能性）
    let content = fs::read_to_string(path)?;
    
    // 内容を解析（InvalidFormatが発生する可能性）
    content.trim().parse::<u32>()
        .map_err(|_| ConfigError::InvalidFormat("数値ではありません".to_string()))
}

// anyhowを使用してResultを簡単に定義(アプリ関数と仮定）
fn get_app_config() -> Result<u32> {
    let config_path = Path::new("config.txt");
    // ライブラリ関数を呼び出し、エラーにコンテキストを追加
    let value = lib_parse_config(config_path)
        .context("アプリケーション設定の読み込みに失敗しました")?;

    Ok(value)
}

fn main() {
    match get_app_config() {
        Ok(value) => println!("設定値: {}", value),
        Err(err) => {
            eprintln!("エラー: {}", err);
            // 根本原因を特定
            if let Some(source) = err.root_cause().downcast_ref::<ConfigError>() {
                match source {
                    ConfigError::IoError(_) => eprintln!("ファイルアクセスに問題があります"),
                    ConfigError::InvalidFormat(_) => eprintln!("設定ファイル形式を確認してください"),
                }
            }
        }
    }
}
