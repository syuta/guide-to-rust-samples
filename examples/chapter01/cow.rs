use std::borrow::Cow;

fn print_string(s: &str) {
    let cow: Cow<str> = Cow::Borrowed(s);
    println!("{}", cow);
}

fn mod_string(mut cow: Cow<str>) -> Cow<str> {
    if cow.as_ref().starts_with("Hello") {
        cow.to_mut().push_str(", Rust!");
    }
    cow
}

fn main() {

    //不変参照
    let s: &str = "Hello, Immutable World!";
    print_string(s);


    //可変参照
    let s: &str = "Hello, Mutable World!";
    let cow: Cow<str> = Cow::Borrowed(s);
    let mod_cow = mod_string(cow);
    println!("{}", mod_cow);

    //mod_cowは、Cow::BorrowedなのでCloneが作られる
    let mod_cow_cloned = mod_cow.into_owned();

    println!("{}", mod_cow_cloned);
}
