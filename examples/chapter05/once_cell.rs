use once_cell::sync::OnceCell;

static DATABASE: OnceCell<Database> = OnceCell::new();

struct Database {
    // DB接続情報など
    url: String,
}

impl Database {
    fn new(url: &str) -> Self {
        println!("DB接続を初期化中...");
        // 実際の接続処理
        Self { url: url.to_string() }
    }
}

fn initialize_db(url: &str) {
    // 一度だけ初期化
    DATABASE.get_or_init(|| Database::new(url));
}

fn get_db() -> &'static Database {
    DATABASE.get().expect("DBが初期化されていません")
}

fn main() {
    // アプリケーション起動時に初期化
    initialize_db("postgres://localhost/hogedb");
    
    // 他の場所から安全にアクセス
    let db = get_db();
    println!("DB URL: {}", db.url);
}
