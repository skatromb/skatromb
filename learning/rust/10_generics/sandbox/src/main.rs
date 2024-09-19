fn main() {
    // let string1 = String::from("long string is long");
    // let result: &str;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
    let string2 = create_str();
    println!("{string2}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn create_str() -> String {
    String::from("really long string")
}
