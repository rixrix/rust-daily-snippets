// A static variable has fixed address in memory
static mut COUNTER: u32 = 0;

fn increment_static_variable(inc: u32) {
    // accessing and modifying a mutable static variable is deemed unsafe
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    increment_static_variable(5);
    unsafe {
        assert!(COUNTER == 5);
        println!("Counter: {}", COUNTER);
    }

    increment_static_variable(10);
    unsafe {
        assert!(COUNTER == 15);
        println!("Counter: {}", COUNTER);
    }
}
