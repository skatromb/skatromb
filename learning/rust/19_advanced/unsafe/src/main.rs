unsafe fn dangerous() {}
fn safe_function() {
    unsafe { dangerous() }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x00010usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
    
    unsafe { dangerous() };
    safe_function();
}

