// This function uses Option enum type that represents the absence or presence of a value
fn safe_division(num: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(num / divisor)
    }
}
fn main() {
    let num = 10;
    let divisor = 3000;

    // Uses match to check for the value. It's an equivalent of `switch` statement for other languages
    match safe_division(num, divisor) {
        None => println!("Division is Zero!"),
        Some(result) => println!("{} is the result of {}/{}", result, num, divisor),
    }

    match safe_division(0, 0) {
        None => println!("Zero!"),
        Some(result) => println!("result: {}", result),
    }
}
