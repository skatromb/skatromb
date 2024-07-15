struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x:5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 1, y: 2.0 };
    let p = Point {x: 5, y: 10};
    
    println!("p.x = {}", p.x());
}