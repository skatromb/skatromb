#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self { 
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(us_state) => {
                println!("Coin from {:?}" , us_state);
                25
            },
        }
    }
}

fn main() {
    println!("{}", Coin::Quarter(UsState::Alabama).value_in_cents());
}
