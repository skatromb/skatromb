use std::ops::Deref;

#[derive(Debug, PartialEq)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    let z = MyBox(x);

    dbg!(&y);
    dbg!(z.deref());

    assert_eq!(5, x);
    assert_eq!(y, z);
    assert_eq!(5, *y);
    
    let name = MyBox("Rust".to_string());
    hello(&name);
    hello(&(*name)[..]);
}