use std::sync::Once;

struct Logger {
    log_level: u32,
}

// グローバルなインスタンス
static mut LOGGER: Option<Logger> = None;
static INIT: Once = Once::new();

impl Logger {
    // シングルトンインスタンスを取得
    fn instance() -> &'static Logger {
        // 初期化を一度だけ実行する
        INIT.call_once(|| {
            unsafe {
                // 安全な初期化（一度だけ実行）
                LOGGER = Some(Logger { log_level: 3 });
            }
            println!("initialize Logger");
        });
        
        // 初期化済みのインスタンスを返す
        unsafe { LOGGER.as_ref().unwrap() }
    }
    
    fn log(&self, message: &str) {
        println!("[Level : {}] {}", self.log_level, message);
    }
}

fn main() {
    // どこからでも同じインスタンスにアクセス
    let logger1 = Logger::instance();
    logger1.log("first message");
    
    let logger2 = Logger::instance();
    println!("isSame instance? : {}", 
             std::ptr::eq(logger1, logger2)); // true
}
