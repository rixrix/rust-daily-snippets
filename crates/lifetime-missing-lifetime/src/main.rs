// Ref: https://rustlabs.github.io/docs/rust101/lifetimes/

#![allow(dead_code)]
#[derive(Debug)]
struct Course {
    name: String,
    id: i32
}

fn get_course(c1: &Course, c2: &Course) -> &Course {
    if c1.name == "Rust" {
        c1
    } else {
        c2
    }
}

fn main() {
    let c1: Course = Course {
        name: String::from("Rust"),
        id: 101
    };

    let c2: Course = Course {
        name: String::from("C++"),
        id: 103
    };

    let result = get_course(&c1, &c2);
    println!("{:?}", result);
}