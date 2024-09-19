struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}
