mob front_of_house {
    // by default a private child is private and everything
    // thats inside of it from the perspective of the parent module
    // modules, and functions within it must be public
    pub mob hosting {
        pub fn add_to_waitlist() {}
    }
}

// use create, and specify path
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    // absolute path
    // starts at root of our module crate with crate
    //
    //create::front_of_house::hosting::add_to_waitlist();

    // relative path
    // start from the current module
    //
    // front_of_house::hosting::add_to_waitlist();
    //
    // since we are using pub use. we can simply use hosting
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}


// in rust,Use use to bring items into scope.
//
//Use mod to declare a module.
//
//  Use pub to export things (make them public).
//
//  Rust uses file or folder names to infer module names, and as long as the names line up, and items are marked pub, it will work
//
//
//  we can declare modules in rust using mod keyword, and we can define contents of our modules
//  using curly brackets, but if we want the content of our modules be defined in a different file
//  then we only use semicolon  at the end of the name of module, and put content of the module
//  in a file with the same name as the module, similarly we can do with child modules, but for
//  children modules, we must have the file, with the name of the child module within a directory
//  that has the same name as the parent module
