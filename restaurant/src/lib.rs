

fn deliver_order() {}

pub mod customer {

    //By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope
      
    use crate::back_of_house;

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        //meal.seasonal_fruit = String::from("blueberries");
        println!("I'd like {} please", meal.seasonal_fruit);

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;

        println!("order 1: {:?}, order 2: {:?}", order1, order2);

        super::hosting::add_to_waitlist();
    }
}

mod front_of_house;

use crate::front_of_house::hosting;


pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); //uses the parents module's deliver_order()
    }

    pub fn cook_order() {}

    pub struct Breakfast { // pub for all the attributes you want public
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer { // make the entire enumeration public
        Soup,
        Salad,
    }
}