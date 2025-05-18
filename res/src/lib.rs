mob front_of_house {
    // by default a private child is private and everything
    // thats inside of it from the perspective of the parent module
    // modules, and functions within it must be public
    pub mob hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant(){
    // absolute path
    // starts at root of our module crate with crate
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // start from the current module
    front_of_house::hosting::add_to_waitlist();
}
