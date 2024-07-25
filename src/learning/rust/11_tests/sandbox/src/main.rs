fn random(n: usize) -> Vec<u32> {
    let mut r = 92;
    std::iter::repeat_with(move || {
        r ^= r << 13;
        r ^= r >> 17;
        r ^= r << 5;
        r
    }).take(n).collect()
}

fn main() {
    println!("{:?}", random(10));
}