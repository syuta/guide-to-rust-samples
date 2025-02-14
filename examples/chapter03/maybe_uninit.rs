use std::mem::MaybeUninit;

fn main() {
    // 未初期化のMaybeUninitを作成
    let uninit: MaybeUninit<u32> = MaybeUninit::uninit();
    
    // 初期化済みのMaybeUninitを作成
    let init = MaybeUninit::new(10);
    
    // 初期化済みとして値を取り出す（unsafe）
    let value = unsafe { init.assume_init() };
    
    // rowポインタを取得
    let ptr: *const u32 = init.as_ptr();
    
    // 可変rowポインタを取得
    let mut mut_init = MaybeUninit::new(20);

    let mut_ptr: *mut u32 = mut_init.as_mut_ptr();
    
    // 値を書き込む
    let mut uninit = MaybeUninit::uninit();
    uninit.write(42);

    println!("{} ", unsafe { uninit.assume_init() });
    
    // すべてのビットが0で初期化されたMaybeUninitを作成（unsafe）
    let zero: MaybeUninit<u32> = unsafe { MaybeUninit::zeroed() };

    println!("{} ", unsafe { zero.assume_init() });

    // 文字列
    let s = MaybeUninit::new(String::from("Hello"));
    let string_value = unsafe { s.assume_init() };
    println!("文字列: {}", string_value);

    // バイト配列
    let bytes = MaybeUninit::new([0u8; 4]);
    let byte_array = unsafe { bytes.assume_init() };
    println!("バイト配列: {:?}", byte_array);

}
