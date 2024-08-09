fn main() {
    let string: String = ('a'..='z').chain('0'..='5').collect();
    println!("{string:?}");
}