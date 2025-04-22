fn main() {
    let start = std::time::Instant::now();
    
    // 計算負荷の高い処理
    let mut sum:i64 = 0;
    for i in 0..10_000_0 {
        sum += i;
    }
    
    let duration = start.elapsed();
    println!("結果: {}, 処理時間: {:?}", sum, duration);
}
