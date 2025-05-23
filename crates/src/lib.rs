// think of modules as the folders in your computer,
// to explain this, right here in this res folder
// it's a create. It has one children, front_of_house
// which has two modules, hosting, and serving, hosting has 2 functions
// and servining has 3 functions
// the tree would be like this
/*
crate
|___front_of_house
    |
    |__hosting
    |   |
    |   |__add_to_waitlist
    |   |__seat_at_table
    |
    |__serving
        |
        |__take_order
        |__serve_order
        |__take_payment
 */



//n now here we are bringing hosting, by bringing front of house,
//create a file with the name of this module, then a folder, with the same name, and the funciton
// create a file with the name of the function
mod front_of_house;

mod front_of_house {
    mod hosting {
        fn add_to_waitlist(){}

        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

