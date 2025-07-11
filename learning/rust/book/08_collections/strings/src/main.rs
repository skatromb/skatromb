enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let _ = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text("blue".to_string()),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = "foo".to_string();
    let s2 = "bar";
    s.push_str(s2);

    println!("s2 is {s2}");

    let mut s2 = "lo".to_string();
    s2.push('l');
    println!("{s2}");

    let s3 = s + &s2;
    println!("{s3}");

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{b}");
    }
    println!("{}", hello);
}
