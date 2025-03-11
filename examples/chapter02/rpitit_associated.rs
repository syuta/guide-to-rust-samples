trait DataConverter {
    // 関連型で入力型を定義
    type Input;

    // RPITITで戻り値の型を抽象化
    fn convert(&self, input: Self::Input) -> impl Iterator<Item = String>;
}

// 数値を文字列に変換する構造体
struct NumberConverter;

impl DataConverter for NumberConverter {
    type Input = i32;

    fn convert(&self, input: Self::Input) -> impl Iterator<Item = String> {
        // 数値とその2倍の値を文字列として返す
        vec![input.to_string(), (input * 2).to_string()].into_iter()
    }
}

fn main() {
    let converter = NumberConverter;
    for result in converter.convert(5) {
        println!("Converted: {}", result);
    }
}
