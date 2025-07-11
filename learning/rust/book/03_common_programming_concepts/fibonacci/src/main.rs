use std::io;

// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233
fn fib(n: u16) -> u32 {
    let mut elem: u32 = 0;
    let mut elem_next: u32 = 1;
    for _ in 0..n-1 {
        (elem, elem_next) = (elem_next, elem + elem_next);
    };
    elem
}

fn main() {
    println!("Please enter Fibonacci number you like to see:");
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse() {
                Ok(num) => println!("Fibonacci number: {}", fib(num)),
                Err(e) => println!("Error {e}"),
            };
        },
        Err(e) => println!("Error {e}"),
    }

}
