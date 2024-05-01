// This won't compile.
// This function is returning a reference that is only available inside this function
fn format_to_string(data: &i32) -> &str {
    let s = format!("{}", &data);
    &s // dangling reference
}
fn main() {
    let num = 90;
    let result = format_to_string(&num);
    println!("Result: {}", result);
}
