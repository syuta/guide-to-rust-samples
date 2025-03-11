fn main() {
    //構造体のフィールドを無視する
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let point = Point { x: 1, y: 2, z: 3 };

    let Point { x, .. } = point;
    println!("構造体から x だけ取り出し: {}", x);

    match point {
        p @ Point { x: 0, .. } => println!("x軸上にあります: {:?}", p),
        p @ Point { y: 0, .. } => println!("y軸上にあります: {:?}", p),
        p @ Point { .. } => println!("その他の位置にあります: {:?}", p),
    }

    //タプルの一部要素だけ使う
    let tuple = (1, 2, 3, 4, 5);

    let (first, .., last) = tuple;
    println!("タプルの最初: {}, 最後: {}", first, last);

    match tuple {
        t @ (1, ..) => println!("1で始まるタプル: {:?}", t),
        t @ (.., 5) => println!("5で終わるタプル: {:?}", t),
        other => println!("その他のタプル: {:?}", other),
    }

    let (a, b, ..) = tuple;
    println!("タプルの最初の2つ: {}, {}", a, b);

    // 配列やスライスの一部だけマッチさせる
    let arr = [1, 2, 3, 4, 5];

    let [first, rest @ ..] = &arr;
    println!("配列の最初: {}, 残り: {:?}", first, rest);

    match &arr {
        full @ [first, .., last] => {
            println!("配列の最初: {}, 最後: {}", first, last);
            println!("配列全体: {:?}", full);
        }
    }

    match &arr {
        pattern @ [1, 2, ..] => println!("1, 2で始まる配列: {:?}", pattern),
        _ => println!("その他の配列"),
    }

    // 範囲パターン
    for i in 0..10 {
        match i {
            n @ 0..=3 => println!("{}: 0〜3の範囲内", n),
            n @ 4..=7 => println!("{}: 4〜7の範囲内", n),
            n @ _ => println!("{}: 8以上", n),
        }
        if i >= 5 {
            break;
        }
    }

    #[derive(Debug)]
    enum Message {
        Move { x: i32, y: i32 },
        Write(String),
        Color(u8, u8, u8),
    }

    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("Hello"));

    // @演算子を使ったパターンマッチ
    match msg1 {
        // 値全体をmに束縛し、xも取り出す
        m @ Message::Move { x, .. } => {
            println!("移動: x={}, 全体={:?}", x, m)
        }
        _ => println!("その他のメッセージ"),
    }

    // refキーワードを使って所有権の移動を防ぐ
    match msg2 {
        // refを使って参照として束縛
        ref m @ Message::Write(ref text) => {
            println!("テキスト: {}, 全体={:?}", text, m)
        }
        _ => println!("その他のメッセージ"),
    }

    // 参照パターンでの束縛
    let reference = &42;
    match reference {
        r @ &val => println!("参照値: {}, 参照自体: {:p}", val, r),
    }
}
