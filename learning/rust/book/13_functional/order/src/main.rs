#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
        Rectangle { width: 10, height: 1 },
    ];

    let mut num_sort_operations: u16 = 0;
    list.sort_unstable_by_key(|r| {
        num_sort_operations += 1;
        r.width * r.height
    });
    println!("{list:#?}");
    println!("{num_sort_operations:#?}");
}
