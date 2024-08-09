mod blah;

fn deliver_order() {}

mod back_of_house;


mod front_of_house;
mod customer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
