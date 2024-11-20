// 共通のトレイト
trait HasPrice {
    fn price(&self) -> u32;
    fn discount(&self) -> u32 {
        self.price() / 10 // デフォルト実装：10%引き
    }
}

// 車の構造体
struct Car {
    model: String,
    cost: u32,
}

// 本の構造体
struct Book {
    title: String,
    pages: u32,
    base_price: u32,
}

// 車にHasPriceを実装
impl HasPrice for Car {
    fn price(&self) -> u32 {
        self.cost
    }
}

// 本にHasPriceを実装
impl HasPrice for Book {
    fn price(&self) -> u32 {
        self.base_price + self.pages // ページ数に応じて価格が上がる
    }
}

fn main() {
    let car = Car {
        model: String::from("Sedan"),
        cost: 20000,
    };

    let book = Book {
        title: String::from("Rust Programming"),
        pages: 300,
        base_price: 2000,
    };

    println!("Car price: {}, discount: {}", car.price(), car.discount());
    println!(
        "Book price: {}, discount: {}",
        book.price(),
        book.discount()
    );
}
