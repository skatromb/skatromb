pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// # Examples
///
/// ```
///
/// let result = adder::add_two(2);
/// assert_eq!(result, 4);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }
    
    // #[test]
    // fn failing() {
    //     panic!("Make this test fail");
    // }
}
