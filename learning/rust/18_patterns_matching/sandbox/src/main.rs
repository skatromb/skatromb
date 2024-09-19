struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x: u8 = 5;

    match x {
        1..=5 => println!("blah"),
        6..=u8::MAX => {},
        0 => {panic!()},
    }
    println!("{}", char::MAX);

    let p = Point {x: 0, y: 7};

    let Point { x, y} = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);

    let p = Point {x: 0, y: 7};

    match p {
        Point { x, y: 0} => println!("One the x axis at {x}"),
        Point { x: 0, y} => println!("On the y axis at {y} and x is {x}"),
        Point {x, y} => println!("On neither axis: ({x}, {y})"),
    }
}
