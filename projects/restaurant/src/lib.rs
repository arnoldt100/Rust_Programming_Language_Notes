mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table () {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payments() {}
    }
}

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
            pub fn summer(toast: &str)-> Breakfast {
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

pub fn deliver_order() {}

mod customer {
    use super::front_of_house::hosting;
    use crate::back_of_house::Breakfast;
    use crate::back_of_house::Appetizer;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::seat_at_table();

        // Order a breakfast meal with rye toast.
        let mut meal = Breakfast::summer("Rye");

        // Change our mind.
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please.", meal.toast);

        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
    }
}

