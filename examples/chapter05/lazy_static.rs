use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CONFIG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("host", "localhost");
        map.insert("port", "8080");
        map.insert("user", "admin");
        // 実行時の環境変数から値を取得することも可能
        if let Ok(debug) = std::env::var("DEBUG") {
            map.insert("debug", if debug == "true" { "enabled" } else { "disabled" });
        }
        map
    };
}

fn main() {
    // 初回アクセス時に初期化される
    println!("設定: {:?}", *CONFIG);
    println!("ホスト: {}", CONFIG.get("host").unwrap_or(&"unknown"));
}
