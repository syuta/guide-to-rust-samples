use std::error::Error;
use std::fmt;

// カスタムエラー型の定義
#[derive(Debug)] 
enum OrderError {
    OutOfStock { product_id: u32 },
    InsufficientFunds { balance: f64, price: f64 },
}

// Display トレイトの実装（ユーザー向けエラーメッセージ）
impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderError::OutOfStock { product_id } => 
                write!(f, "商品 #{} は在庫切れです", product_id),
            OrderError::InsufficientFunds { balance, price } => 
                write!(f, "残高不足です（残高: {:.2}円, 必要金額: {:.2}円）", balance, price),
        }
    }
}

// Error トレイトの実装
impl Error for OrderError {}

// 注文処理を行う関数
fn process_order(product_id: u32, user_balance: f64) -> Result<String, OrderError> {
    // 在庫チェック
    if product_id == 0 {
        return Err(OrderError::OutOfStock { product_id });
    }
    
    // 残高チェック
    let price = 1000.0;
    if user_balance < price {
        return Err(OrderError::InsufficientFunds { 
            balance: user_balance, 
            price 
        });
    }
    
    Ok(format!("商品 #{} の注文が完了しました", product_id))
}

fn main() {
    // 複数ケースのテスト
    let cases = [
        (0, 2000.0),  // 在庫切れ
        (1, 500.0),   // 残高不足
        (1, 2000.0),  // 成功
    ];
    
    for (product_id, balance) in cases {
        match process_order(product_id, balance) {
            Ok(message) => println!("成功: {}", message),
            Err(e) => println!("失敗: {}", e),
        }
    }
}
