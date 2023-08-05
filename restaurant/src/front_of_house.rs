pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

pub mod serving {
    pub fn take_order() {}

    pub fn serve_order() {}

    pub fn take_payment() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    self::hosting::add_to_waitlist();

    crate::front_of_house::serving::take_order();
}