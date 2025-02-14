use std::convert::TryFrom;

#[derive(Debug)]
struct EvenNumber(i32);

// TryFromを実装
impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("奇数は変換できません")
        }
    }
}

fn main() {
    // TryFromを使用
    let even = EvenNumber::try_from(8);
    println!("TryFromの結果: {:?}", even);  // Ok(EvenNumber(8))

    // TryIntoを使用（自動的に実装されている）
    let result: Result<EvenNumber, _> = 8_i32.try_into();
    println!("TryIntoの結果: {:?}", result);  // Ok(EvenNumber(8))
    
    // 変換が失敗するケース
    let odd = EvenNumber::try_from(5);
    println!("奇数の変換: {:?}", odd);   // Err("奇数は変換できません")
    
    let odd_result: Result<EvenNumber, _> = 5_i32.try_into();
    println!("奇数のTryInto: {:?}", odd_result);  // Err("奇数は変換できません")
}
