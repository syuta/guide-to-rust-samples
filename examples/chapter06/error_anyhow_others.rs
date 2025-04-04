use anyhow::{Context, Result};

fn parse_config(content: &str) -> Result<i32> {
    content.trim().parse::<i32>()
        .context("設定値を整数として解析できませんでした")
}

fn main() {
    let result = std::fs::read_to_string("config.txt")
        .context("設定ファイルの読み取りに失敗")
        .and_then(|content| parse_config(&content));

    match result {
        Ok(value) => println!("設定値: {}", value),
        Err(e) => {
            // 簡潔な表示
            println!("エラー: {}", e);
            // 詳細なエラーチェーンの表示
            println!("詳細: {:#}", e);
            // 手動でエラーチェーンを辿る場合
            let mut source = e.source();
            while let Some(cause) = source {
                println!("原因: {}", cause);
                source = cause.source();
            }
        }
    }
}
