mod data {
    // 非公開関数（dataモジュール内でのみアクセス可）
    fn internal_process() {
        println!("内部処理");
    }
    
    // 公開している関数（モジュール外からアクセス可）
    pub fn process() {
        // 同じモジュール内の非公開関数にアクセス可
        internal_process();
    }
    
    // 公開している構造体
    pub struct User {
        pub name: String, // 公開フィールド
        password: String, // 非公開フィールド
    }
    
    impl User {
        pub fn new(name: String, password: String) -> User {
            User { name, password }
        }
        
        pub fn authenticate(&self, input: &str) -> bool {
            self.password == input
        }
    }
}

fn main() {
    data::process();
    // data::internal_process();  // エラー: private function
    
    let user = data::User::new("taro".to_string(), "password1234".to_string());
    println!("name: {}", user.name);
    println!("auth: {}", user.authenticate("password1234"));
    // println!("password: {}", user.password);  // エラー: private field
}
