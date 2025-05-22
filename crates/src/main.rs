// modules privary
//
mod back_of_house {

    // structs have to be public, as well as their fields
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal: Breakfast = back_of_house::Breakfast::summer("rye");

    meal.toast = String::from("somehting else");
}
