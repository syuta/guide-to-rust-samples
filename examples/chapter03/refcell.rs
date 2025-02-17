use std::cell::RefCell;

fn main() {
    // RefCellを作成（初期値はString）
    let cell = RefCell::new(String::from("Hello"));

    // 不変参照を取得
    {
        let borrowed = cell.borrow();
        println!("Initial value (borrow): {}", *borrowed);
        // borrowedはこのスコープ内で有効
    } // ここでborrowが解放される

    // 可変参照を取得して値を変更
    {
        let mut borrowed_mut = cell.borrow_mut();
        borrowed_mut.push_str(", RefCell");
        println!("After mutable borrow: {}", *borrowed_mut);
        // borrow_mutの終了で変更はRefCell内部に反映される
    }

    // 可変参照取得を試みる（失敗した場合はErrを返す）
    {
        match cell.try_borrow_mut() {
            Ok(mut bm) => {
                bm.push_str("!!!");
                println!("After try_borrow_mut: {}", *bm);
            }
            Err(e) => println!("try_borrow_mut failed: {:?}", e),
        }
    }

    // 内部の値をまとめごと置き換え、古い値を返す
    {
        let old_value = cell.replace(String::from("New value"));
        println!("Replaced old value: {}", old_value);
        println!("Current value after replace: {}", cell.borrow());
    }

    //RefCell自体の所有権を奪い、中の値を取得
    {
        let cell2 = RefCell::new(42);
        let inner_value = cell2.into_inner(); // cell2はここで消費される
        println!("Inner value via into_inner: {}", inner_value);
    }
}
