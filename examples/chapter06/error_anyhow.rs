use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn load_config() -> Result<i32> {
    // ファイルを開く - エラー時にコンテキスト情報を追加
    let mut file = File::open("config.txt")
        .context("can not open file")?;
    
    // ファイル内容を読み込む
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("can not read file")?;
    
    // 内容を解析
    let value = contents.trim().parse::<i32>()
        .context("number parse error")?;
    
    // 条件チェック(設定値は0より上)
    if value < 0 {
        anyhow::bail!("Set value must be greater than 0");
    }
    
    Ok(value)
}

// main関数でもanyhow::Resultを使用できる
fn main() -> Result<()> {
    let config_value = load_config()?;
    println!("setting value: {}", config_value);
    Ok(())
}
