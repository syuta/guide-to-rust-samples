// データの読み取り機能
trait Readable {
    fn read(&self) -> String;
}

// データの読み取りと解析機能
trait Parseable: Readable {
    fn parse(&self) -> Result<Vec<String>, String> {
        // read()メソッドが利用可能であることが保証されている
        let content = self.read();
        // contentを解析して結果を返す
        Ok(content.split(',').map(String::from).collect())
    }
}

struct Data {
    data: String,
}

impl Readable for Data {
    fn read(&self) -> String {
        self.data.clone()
    }
}

impl Parseable for Data {}

fn main() {
    let data = Data {
        data: "my data1,my data2,my data3".to_string()
    };

    let read_data = data.read();
    println!("read_data: {}", read_data);

    let parsed_data = data.parse();
    print!("parsed_data: {:?}" , parsed_data);
}
