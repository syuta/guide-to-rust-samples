pub mod models {
    // 公開構造体
    pub struct User {
        pub username: String,  // 公開フィールド
        email: String,         // 非公開フィールド
    }
    
    impl User {
        pub fn new(username: String, email: String) -> Self {
            User { username, email }
        }
        
        pub fn email(&self) -> &str {
            &self.email
        }
    }
    
    // 公開トレイト
    pub trait Displayable {
        fn display(&self) -> String;
    }
    
    impl Displayable for User {
        fn display(&self) -> String {
            format!("ユーザー: {} ({})", self.username, self.email)
        }
    }
}

fn main() {

    let user = models::User::new("yamada".to_string(), "yamada@example.com".to_string());
    
    println!("ユーザー名: {}", user.username);  // OK: 公開フィールド
    // println!("メール: {}", user.email);      // エラー: 非公開フィールド
    println!("メール: {}", user.email());       // OK: 公開メソッド経由

    // トレイトをスコープに入れる
    use models::Displayable;
    println!("{}", user.display());             // OK: トレイトメソッド
}