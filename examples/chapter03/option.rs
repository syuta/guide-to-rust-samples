fn main() {
    // 基本的な使い方
    let some_num = Some(42);
    let _none_num: Option<i32> = None;

    // matchを使用
    match some_num {
        Some(n) => println!("値: {}", n),
        None => println!("値が存在しません"),
    }

    // if letでシンプルに書ける
    if let Some(n) = some_num {
        println!("数値: {}", n);
    }

    // Temperature enumの例
    #[derive(Debug)]
    enum Temperature {
        Celsius(f64),
        Fahrenheit(f64),
        Kelvin(f64),
    }

    let temp = Temperature::Celsius(25.0);

    if let Temperature::Celsius(c) = temp {
        println!("摂氏{}度です", c);
    } else if let Temperature::Fahrenheit(f) = temp {
        println!("華氏{}度です", f);
    } else if let Temperature::Kelvin(k) = temp {
        println!("{}ケルビンです", k);
    } else {
        println!("不明な温度単位です");
    }

    // よく使う関数の例
    let opt = Some(10);

    // unwrap関連
    let value = opt.unwrap(); // 10
    println!("unwrap: {}", value);

    let value = opt.unwrap_or(0); // 10
    println!("unwrap_or: {}", value);

    let none_value: Option<i32> = None;
    let value = none_value.unwrap_or(0); // 0
    println!("none unwrap_or: {}", value);

    // unwrap_or_else
    let value = opt.unwrap_or_else(|| 42);
    println!("unwrap_or_else: {}", value);

    // map関連
    let mapped = opt.map(|x| x * 3); // Some(30)
    println!("map: {:?}", mapped);

    let x_none: Option<i32> = None;
    let y = x_none.map_or(42, |n| n * 2); // 42
    println!("map_or: {}", y);

    // map_or_else
    let y = x_none.map_or_else(|| 42, |n| n * 2);
    println!("map_or_else: {}", y);

    // 値の存在確認
    let opt = Some(42);
    if opt.is_some() {
        println!("値が存在します");
    }
    if opt.is_none() {
        println!("値が存在しません");
    }

    // and_then, or_else
    let x = Some(5);
    let y = x.and_then(|n| if n > 0 { Some(n * 2) } else { None });
    println!("and_then: {:?}", y);

    let x: Option<i32> = None;
    let y = x.or_else(|| Some(10));
    println!("or_else: {:?}", y);

    // filter, and, or
    let x = Some(5);
    let y = x.filter(|n| n > &3);
    let z = x.filter(|n| n > &10);
    println!("filter: {:?}, {:?}", y, z);

    let x = Some(2);
    let y = Some(3);
    let z = x.and(y);
    println!("and: {:?}", z);

    let x: Option<i32> = None;
    let y = Some(3);
    let z = x.or(y);
    println!("or: {:?}", z);

    // copied と cloned
    let x: Option<&i32> = Some(&5);
    let y: Option<i32> = x.copied();
    println!("copied: {:?}", y);

    // clonedの例（修正版）
    let hello = String::from("hello"); // 先に変数を作成
    let x: Option<&String> = Some(&hello);
    let y: Option<String> = x.cloned();
    println!("cloned: {:?}", y);

    // イテレータとの組み合わせ
    let numbers = vec![1, 2, 3];
    let first: Option<i32> = numbers.first().copied();
    println!("first: {:?}", first);

    let found: Option<i32> = numbers.iter().find(|&&x| x == 2).copied();
    println!("found: {:?}", found);
}
