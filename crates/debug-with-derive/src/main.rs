/// If you comment out `derive` Rust will give you a hint on what to do
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let user = Person {
        name: "Rix".to_string(),
        age: 20,
    };
    println!("New user {:#?}", user);
}
