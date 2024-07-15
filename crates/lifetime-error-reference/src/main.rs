// Ref: https://rustlabs.github.io/docs/rust101/lifetimes/

#![allow(dead_code)]
struct Course {
    name: String,
    id: i32
}

fn main() {
    let c1: &Course;
    {
        let c2: Course = Course {
            name: String::from("Rust!"),
            id: 101
        };
    }

    // referencing a resource that is deallocated
    // here c2 is dropped and scope is not available
    c1 = &c2;
}