pub fn log(message: &str) {
    // タイムスタンプ機能（timestamps機能が有効な場合）
    #[cfg(feature = "timestamps")]
    let prefix = {
        use chrono::Local;
        format!("[{}] ", Local::now().format("%H:%M:%S"))
    };

    #[cfg(not(feature = "timestamps"))]
    let prefix = String::new();

    // カラー出力機能（colored機能が有効な場合）
    #[cfg(feature = "colored")]
    let formatted = format!("\x1b[32m{}{}\x1b[0m", prefix, message);

    #[cfg(not(feature = "colored"))]
    let formatted = format!("{}{}", prefix, message);

    println!("{}", formatted);
}
