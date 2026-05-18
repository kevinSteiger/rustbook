
mod front_of_house;

fn deliver_order() {}

mod back_of_house{

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


pub use crate::front_of_house::hosting;
mod customer{
    use crate::front_of_house::hosting;
    use crate::back_of_house::Appetizer;
    use crate::back_of_house::Breakfast;

    pub fn eat_at_restaurant() {

        let order1 = Appetizer::Soup;

        let mut meal = Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast", meal.toast);

        hosting::add_to_waitlist();
        
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}




use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);


    let secret_number = rand::thread_rng().gen_range(1..=100);
}

fn function1() -> Result {
    Ok(())
}
fn function2() -> IoResult<()> {
    Ok(())
}