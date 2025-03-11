// コレクション操作を定義するトレイト
trait MyCollection<T> {
    type MyIter<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
    fn iter(&self) -> Self::MyIter<'_>;
    fn add(&mut self, item: T);
}

// Vec<T>をラップした構造体
struct SimpleCollection<T> {
    items: Vec<T>,
}

// SimpleCollection<T>にCollectionトレイトを実装
impl<T> MyCollection<T> for SimpleCollection<T> {
    type MyIter<'a>
        = std::slice::Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::MyIter<'_> {
        self.items.iter()
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

// コレクションを処理する汎用関数
fn process_collection<C, T>(collection: &C)
where
    C: MyCollection<T>,
    T: std::fmt::Display,
{
    println!("Processing collection:");
    for item in collection.iter() {
        println!("Item: {}", item);
    }
}

fn main() {
    // 数値のコレクション
    let mut numbers = SimpleCollection { items: Vec::new() };
    numbers.add(1);
    numbers.add(2);
    numbers.add(3);

    // 文字列のコレクション
    let mut words = SimpleCollection { items: Vec::new() };
    words.add(String::from("Hello"));
    words.add(String::from("World"));

    // 両方のコレクションを処理
    process_collection(&numbers);
    println!("---");
    process_collection(&words);
}
