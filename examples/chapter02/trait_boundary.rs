use std::fmt::Debug;

// Printableトレイトの定義
trait Printable {
    fn print(&self);
}

// Book構造体の定義
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

// BookにPrintableを実装
impl Printable for Book {
    fn print(&self) {
        println!("Book: {} by {}", self.title, self.author);
    }
}

// Magazine構造体の定義
#[derive(Debug)]
struct Magazine {
    name: String,
    issue: u32,
}

// MagazineにPrintableを実装
impl Printable for Magazine {
    fn print(&self) {
        println!("Magazine: {} Issue #{}", self.name, self.issue);
    }
}

// 複数のトレイト境界を持つジェネリック関数
fn describe<T: Printable + Debug>(item: T) {
    println!("Debug view: {:?}", item);
    item.print();
}

// トレイト境界を持つジェネリック構造体
struct Container<T: Printable> {
    item: T,
}

impl<T: Printable> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn print_contents(&self) {
        self.item.print();
    }
}

fn main() {
    let book = Book {
        title: String::from("Guide to Rust"),
        author: String::from("Nakamura Shuta"),
    };

    describe(book);

    let magazine = Magazine {
        name: String::from("Rust Magazine"),
        issue: 16,
    };

    let container = Container::new(magazine);
    container.print_contents();
}
