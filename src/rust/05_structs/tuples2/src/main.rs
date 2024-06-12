#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let square = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels.",
        square.area()
        );
    println!("{:?}", rect1);
    dbg!(&rect1);
    dbg!(30 * rect1.width);
    println!("{:?}", square);
    println!("rect1 can hold square? {}", rect1.can_hold(&square))
}
