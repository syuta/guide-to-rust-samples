use std::fmt;

// 商品用構造体
#[derive(Debug)]
struct Product {
    name: String,
    price: i32,
    stock: i32,
}

// Displayトレイト実装
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (¥{}) - {} in stock",
            self.name, self.price, self.stock
        )
    }
}

fn main() {
    let product = Product {
        name: String::from("Bus Salt"),
        price: 650,
        stock: 20,
    };

    // Debug出力（開発者向け）
    println!("Debug output:");
    println!("{:?}", product);
    println!("{:#?}", product);

    // Display出力（ユーザー向け）
    println!("\nDisplay output:");
    println!("{}", product);
}
