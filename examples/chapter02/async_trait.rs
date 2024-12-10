use tokio;
use std::time::Duration;

// 非同期処理を行うトレイト
trait AsyncProcessor {
    // 非同期メソッド
    async fn process_with_delay(&self, value: i32) -> i32;
}

// 処理を実装する構造体
struct Processor;

impl AsyncProcessor for Processor {
    async fn process_with_delay(&self, value: i32) -> i32 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        value * 3
    }

}

// 複数の非同期処理を実行する関数
async fn run_multiple_processes(processor: &Processor) {
    // 同時に複数の非同期処理を開始
    let process1 = processor.process_with_delay(5);
    let process2 = processor.process_with_delay(10);

    // joinを使用して両方の処理の完了を待機
    let (result1, result2) = tokio::join!(process1, process2);

    println!("Process 1 result: {}", result1);
    println!("Process 2 result: {}", result2);

    let foo = Processor;
    assert_send(foo.process_with_delay(10));
    fn assert_send<T: Send>(_value: T) {}
}

#[tokio::main]
async fn main() {
    let processor = Processor;
    // 非同期処理実行
    run_multiple_processes(&processor).await;
}