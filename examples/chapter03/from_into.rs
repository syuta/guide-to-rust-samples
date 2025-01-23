struct Person {
    name: String,
}

impl From<&str> for Person {
    fn from(name: &str) -> Self {
        Person {
            name: String::from(name)
        }
    }
}
fn main() {
// 使用方法
let person = Person::from("John");
println!("Person name1: {}", person.name);
let person: Person = "John".into();  // Into も自動的に使える
print!("Person name2: {}", person.name);
}