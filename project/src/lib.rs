pub mod back_of_house;
mod front_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::serve_order();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}
