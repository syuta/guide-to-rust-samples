use std::rc::Rc;

fn main() {
    let s_rc = Rc::new(50);

    // Rc::strong_count() - 強い参照の数を取得
    // 参照カウント = 1
    println!("reference count :{}",Rc::strong_count(&s_rc));

    let s_rc_clone = Rc::clone(&s_rc);

    // 参照カウント = 2
    println!("reference count :{}",Rc::strong_count(&s_rc_clone));

    // 値の参照
    println!("Value through original: {}", *s_rc);
    println!("Value through clone: {}", *s_rc_clone);

    // &s_rc == &s_rc_clone
    println!("s_rc = {:p}", s_rc);
    println!("s_rc_clone = {:p}", s_rc_clone);

    // Rc::weak_count() - 弱い参照の数を取得
    let rc = Rc::new(100);
    let w_rc = Rc::downgrade(&rc);
    println!("wrak reference count : {}", Rc::weak_count(&rc));
    
    // weak参照からの値の取得
    if let Some(upgraded) = w_rc.upgrade() {
        println!("upgrade from weak reference: {:?}", *upgraded);
    }
    
    // Rc::try_unwrap() - 参照カウントが1の場合、内部の値を取り出す
    let single_rc = Rc::new(42);
    match Rc::try_unwrap(single_rc) {
        Ok(value) => println!("値の取り出しに成功: {}", value),
        Err(rc) => println!("他の参照が存在します: {}", rc),
    }
}
