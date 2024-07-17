fn main() {
    let string1 = "abcd".to_string();
    let string2 = "xyz";
    
    let result = longest::<&str>(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest<'a, T: PartialOrd>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
