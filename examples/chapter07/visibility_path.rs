mod network {
    // 絶対パスを使用 - network モジュールとその子モジュールからのみアクセス可能
    pub(in crate::network) struct Connection {
        pub host: String,
        pub port: u16,
        auth_token: String,
    }
    
    // network モジュール内の関数
    pub fn connect(host: &str, port: u16, token: &str) -> Connection {
        Connection { 
            host: host.to_string(), 
            port, 
            auth_token: token.to_string() 
        }
    }
    
    // client モジュールを公開
    pub mod client {
        // 相対パスを使用（super は network を指す）
        pub(in super) struct RequestBuilder {
            pub headers: Vec<String>,
        }
        
        // self は client モジュールを指す
        pub(in self) fn validate_request() {
            println!("リクエストを検証中...");
        }
        
        pub fn send_request(path: &str) {
            let conn = super::connect("api.example.com", 443, "secret");
            validate_request();  // client モジュール内でのみアクセス可能
            println!("{}:{} に {} へのリクエストを送信", conn.host, conn.port, path);
        }
    }
    
    pub mod server {
        pub fn handle_connection() {
            // Connection にアクセス可能（network の子モジュールなので）
            let conn = super::connect("0.0.0.0", 8080, "internal");
            println!("{}:{} でリクエストを待機中", conn.host, conn.port);
            
            // RequestBuilder にもアクセス可能（network の子モジュールなので）
            let builder = super::client::RequestBuilder { headers: Vec::new() };
            println!("ヘッダー数: {}", builder.headers.len());
        }
    }
}

fn main() {
    // 公開関数を呼び出し
    network::client::send_request("/api/data");
    network::server::handle_connection();
}