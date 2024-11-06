fn main() {
    // &
    let x = 5;
    let y = &x;  // 不変参照を作成
    println!("x: {}, y: {}", x, y);

    let mut z = 10;
    let w = &mut z;  // 可変参照を作成
    *w += 1;  
    println!("z: {}", z);

    // ref
    match x {
        ref r => println!("Got a refs to x: {}", r),
    }

    match z {
        ref mut mr => {
            *mr += 1;
            println!("z is now: {}", z);
        }
    }

    // &とrefは同じようにも使える
    let a = 100;
    let b = &a;
    let ref c = a;
    println!("a: {}, b: {}, c: {}", a, b, c);
}
