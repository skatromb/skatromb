fn main() {
    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    else {
        println!("No config!");
    }
}
