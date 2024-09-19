use std::ptr::slice_from_raw_parts_mut;
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
     
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid - 1), len - mid + 1),
            )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    
    let r = &mut v[..];
    
    let (a, b) = split_at_mut(r, 3);
    
    // a.iter_mut().for_each(|item| *item = 666);
    *a.last_mut().unwrap() = 666;
    *b.first_mut().unwrap() = 111;
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("a.last() = {:?}", a.last().unwrap());
    
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
