type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}
fn returns_long_type(f: Thunk) {}

fn main() {
    let f: Thunk = Box::new(|| println!("hi"));
    let s2: &str = "How's it going?";
    f();
}
