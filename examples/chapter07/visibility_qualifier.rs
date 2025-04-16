mod outer {
    // クレート全体で公開
    pub(crate) struct Config {
        // 完全に非公開
        api_url: String,
        // outerモジュール内でのみ公開
        pub(self) timeout: u64,
        // 親モジュールでのみ公開
        pub(super) api_key: String,
        // 完全に公開
        pub debug_mode: bool,
    }

    pub mod inner {
        use super::Config;
        pub fn initialize() {
            let config = Config {
                api_url: "https://api.example.com".to_string(),
                timeout: 30,
                api_key: "secret".to_string(),
                debug_mode: true
            };
            
            // api_urlは非公開だが、同じouterモジュール内なのでアクセス可能
            println!("URL: {}", config.api_url);
            
            // timeoutはouter内(self)で公開なのでアクセス可能
            println!("タイムアウト: {}", config.timeout);
            
            // api_keyはsuper(親モジュール)で公開だが、outerモジュール内からもアクセス可能
            println!("APIキー: {}", config.api_key);
            
            // debug_modeは完全に公開なのでアクセス可能
            println!("デバッグモード: {}", config.debug_mode);
        }
    }
    
    // outerモジュール自身のコード
    pub fn setup() {
        let config = Config {
            api_url: "https://api.example.com".to_string(),
            timeout: 30,
            api_key: "secret".to_string(),
            debug_mode: true
        };
        
        // outerモジュール内では全フィールドにアクセス可能
        println!("URL: {}", config.api_url);
        println!("タイムアウト: {}", config.timeout);
        println!("APIキー: {}", config.api_key);
        println!("デバッグモード: {}", config.debug_mode);
    }
}

fn main() {
    // Config構造体はpub(crate)なのでアクセス可能だが、
    // 一部のフィールドは非公開なのでこの方法では初期化できない
    /*
    let config = outer::Config {
        api_url: "https://api.example.com".to_string(),  // エラー: 非公開フィールド
        timeout: 30,                                     // エラー: pub(self)フィールド
        api_key: "secret".to_string(),                   // OK: pub(super)でルートからアクセス可能
        debug_mode: true                                 // OK: 完全に公開
    };
    */
    
    // 代わりに、以下のようなコンストラクタがあれば使用できる
    // let config = outer::Config::new(...);
    
    // 公開関数を通じて間接的にアクセス
    outer::setup();
    outer::inner::initialize();
}