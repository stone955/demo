mod front_of_house;

mod back_of_house;

fn serve_order() {}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relattive path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

    let soup_order = back_of_house::Appetizer::Soup;
    let salad_order = back_of_house::Appetizer::Salad;
}
