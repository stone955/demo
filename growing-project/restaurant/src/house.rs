pub mod back_of_house {
    #[derive(Debug)]
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
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
    }
    fn cook_order() {}
}

pub mod front_of_house {
    pub fn add_to_waitlist() -> bool {
        true
    }

    pub fn seat_at_table() {}

    pub fn take_order() {}

    pub fn serve_order() {}

    pub fn take_payment() {}
}
