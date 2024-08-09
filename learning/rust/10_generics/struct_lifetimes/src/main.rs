struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let s: &str = "blahblah";
    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    
    println!("{}", i.part)
}
