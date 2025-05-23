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

// using enums
//
// enums, unlike structs, we dont have to specify if their variants are private or public
//
mod back_of_house {

    //create public enum
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1: Appetizer = back_of_house::Appetizer::Soup;
    let order2: Appetizer = back_of_house::Appetizer::Salad;
}

// in rust we can use "use" think of it as import in js, it allow us to bring
use create::front_of_house::hosting;

//then use like this
hosting::Soup; //or salad

// we can also use self
use self::front_of_house::hosting;
// functions, modules, traits, structs, enums, etc
// when bringing modules, and functions the idiomatic path should bring the parent function
// however, when bringing things like enums, traits, structs, its better to put the whole path
// the only exception is when you are bringing two enums, with the same name, in that case 
// its better to bring the parent function, we have a context to where it belongs 
// and clarify is not local

