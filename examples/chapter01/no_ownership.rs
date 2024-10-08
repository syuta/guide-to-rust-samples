fn main() {
    let x = 5u32;
    print_number(x);
    println!("x in main: {}", x); // xの所有権は移動していないので使用可能
}

fn print_number(n: u32) {
    println!("Number in function: {}", n);
}
