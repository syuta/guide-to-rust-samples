#[derive(Default, Debug)]
struct Config {
    name: String,        // String implements Default
    count: i32,         // i32 implements Default
    enabled: bool,      // bool implements Default
    tags: Vec<String>,  // Vec<T> implements Default
}

#[derive(Debug)]
struct Config2 {
    name: String,
    count: i32,
    enabled: bool,
    tags: Vec<String>,
}

impl Default for Config2 {
    fn default() -> Self {
        Config2 {
            name: String::from("default_config"),
            count: 10,
            enabled: true,
            tags: vec![String::from("default")],
        }
    }
}

fn main() {
    let config = Config::default();
    println!("{:#?}", config);

    let config2 = Config2::default();
    println!("{:#?}", config2);
}
