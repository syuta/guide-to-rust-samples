fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("foo");
    let string2 = String::from("buzz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}
