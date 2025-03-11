use regex::Regex;

fn main() {
    // 通常の文字列リテラル
    let normal = "Hello\nWorld";

    // 生文字列リテラル
    let raw = r"Hello\nWorld";

    println!("normal string:\n{}", normal);
    println!("raw string:\n{}", raw);

    // 基本的な生文字列
    let raw1 = r"C:\Users\name";

    // ダブルクォートを含む生文字列
    let raw2 = r#"彼は "Rust最高!" と言った"#;

    // 複数のハッシュ記号を使った生文字列
    let raw3 = r##"r#"このように"#を含む文字列も書ける"##;

    println!("{}", raw1);
    println!("{}", raw2);
    println!("{}", raw3);

    // 1つのハッシュ
    let raw4 = r#"r"こんにちは""#;

    // 2つのハッシュ
    let raw5 = r##"r#"これはr#"..."#形式の例です"#"##;

    // 5つのハッシュ
    let raw6 = r#####"複数の"####を含む文字列"#####;

    println!("{}", raw4);
    println!("{}", raw5);
    println!("{}", raw6);

    // 正規表現
    // Raw stringを使うとバックスラッシュをエスケープする必要がない
    let regex_pattern = r"\d{3}-\d{2}-\d{4}"; // 通常なら \\d{3}-\\d{2}-\\d{4}
    let regex = Regex::new(regex_pattern).unwrap();
    let text = "ID: 123-45-6789";
    println!("1. 正規表現:");
    println!("   パターン: {}", regex_pattern);
    println!("   テキスト: {}", text);
    println!("   マッチする?: {}", regex.is_match(text));
    println!();

    // 複数行のJSONデータ
    // 複数行の構造化データを見やすく記述できる
    let json_data = r#"
{
    "name": "Alice",
    "age": 30,
    "languages": ["Rust", "C++", "Python"]
}
"#;
    println!("2. JSON データ:");
    println!("{}", json_data);
    println!();

    // SQLクエリ
    let query = r#"
SELECT users.name, orders.product
FROM users
INNER JOIN orders ON users.id = orders.user_id
WHERE users.active = true AND orders.status = "eable"
"#;
    println!("3. SQL クエリ:");
    println!("{}", query);
    println!();
}
