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
