mod blah;

mod front_of_house {
    pub mod hosting {
        pub(crate) fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}



mod customer {
    use crate::front_of_house::{self, hosting};
    pub use crate::back_of_house::{Appetizer, Breakfast};
    
    pub fn eat_at_restaurant() {
        front_of_house::hosting::add_to_waitlist();
        hosting::add_to_waitlist();

        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
        let mut meal = Breakfast::summer(("Rye"));
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        // prohibited: meal.seasonal_fruit = String::from("blueberries");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
