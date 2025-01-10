trait Transform<T, U> {
    fn transform(&self, input: T) -> impl Iterator<Item = U>;
}

struct Multiplier {
    factor: i32,
}

// i32からf64への変換を実装
impl Transform<i32, f64> for Multiplier {
    fn transform(&self, input: i32) -> impl Iterator<Item = f64> {
        vec![input as f64 * self.factor as f64].into_iter()
    }
}

fn main() {
    let multiplier = Multiplier { factor: 2 };
    for result in multiplier.transform(5) {
        println!("Transformed: {}", result);
    }
}
