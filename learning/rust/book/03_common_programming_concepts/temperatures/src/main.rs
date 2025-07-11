use std::io;

// Let's write a program that will convert Celsius to Fahrenheits
fn main() {
    println!("Please enter temperature in Celsius:");
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let num: Result<f32, _> = input.trim().parse();
            match num {
                Ok(temp_c) => {
                    let temp_f = temp_c * 9.0 / 5.0 + 32.0;
                    println!("Temperature in Fahrenheits would be: {temp_f}");
                },
                Err(error) => println!("Failed to parese input: {error}"),
            }
        },
        Err(error) => println!("Failed to read input {error}")
    }
}
