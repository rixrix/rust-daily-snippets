// Min function is a generic type `T` parameter constrained by `Ord` trait.
// `Ord` defines the behaviour of the types that can be ordered
fn min<T: Ord>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    let a = 100;
    let b = 50;
    let result = min(a, b);
    println!("{} is less than, between {} and {}", result, a, b);
}
