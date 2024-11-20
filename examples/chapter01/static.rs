use std::fmt::Debug;

// 基本的な文字列定数
static APP_NAME: &'static str = "Application";
static VERSION: &'static str = "1.0";

// 構造体での使用
#[derive(Debug)]
struct Config {
    name: &'static str,
    size: i32,
}

static DEFAULT_CONFIG: Config = Config {
    name: "default",
    size: 1000,
};

fn print_app_info(name: &str, version: &str) {
    println!("{} version {}", name, version);
}

fn process_static<T: 'static + Debug>(item: T) {
    println!("item: {:?}", item);
}

fn main() {
    // 定数の使用例
    println!("アプリ名: {}", APP_NAME);
    println!("バージョン: {}", VERSION);

    // 構造体の使用
    println!("デフォルト設定: {:?}", DEFAULT_CONFIG);

    // 定数を関数に渡す
    print_app_info(APP_NAME, VERSION);

    // OK: Stringは参照型ではない
    process_static(String::from("hello"));

    // OK: &'static str は'static参照
    process_static("literal string");

    // エラー: 通常の参照は'staticではない
    //let x = String::from("hello");
    //process_static(&x); // コンパイルエラー
}
