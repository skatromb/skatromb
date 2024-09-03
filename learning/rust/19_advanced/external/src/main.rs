extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of 03 according to C: {}", abs(-3));
    }
}
