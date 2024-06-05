#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        );
    println!("{:?}", rect1);
    dbg!(&rect1);
    dbg!(30 * rect1.width);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
