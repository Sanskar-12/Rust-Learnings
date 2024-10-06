
mod front_of_house;

fn restaurant() {
    crate::front_of_house::hosting::seat_at_table(); // absolute path
    front_of_house::hosting::seat_at_table(); // relative path
}