fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => todo!("hek"),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    
    let none = plus_one(None);
    println!("{:?}", none);
}
