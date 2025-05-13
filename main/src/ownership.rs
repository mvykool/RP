fn main(){
    let s: String = String::from("Hello"); // we declare a variable, type string, which is dynamic,
                                           // and stored in heap
                                           //
    takes_ownership(s);                    // we pass the s variable to this function as a
                                           // parameter, however, keep in mind that passing 
                                           // a variable as a parameter, if its type dynamic its
                                           // the same as if we were moving it to a different
                                           // variable

    println!("{}", s);                     // therefore, this will not run, because the s value
                                          // moved to the function as parameter
}

fn takes_ownership(string_parameter: String) { // since we are moving s variable to this function,
                                               // its now string_parameter variable, once the function
                                               // scope ends, string_parameter is dropped
    println!("{}", string_parameter);
}

// NOW lets do something simliar but with a copy instead of move 

fn main_copy(){
    let x: i32 = 5;

    takes_ownership_copy(x);

    println!("{}", x); //unlike the function above, this one will actually print out a copy, since
                       //takes_ownership_copy copies the variable, it's not moved, just like with
                       //regular variables, sizable data types are copied
}

fn takes_ownership_copy(integer_parameter: i32){
    println!("{}", integer_parameter);
}

// Giving ownership example
fn main_give_ownership(){
    let s1: String = gives_ownership(); //this is a function that returns a String, and moves it to
                                        //the s1 variable, when its called, it returns it, instead
                                        //of dropping it, moving it to s1, which now can be called
    println!("{}", s1);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");

    some_string
}

//taking and giving ownership

//this is an example of how we can take, and give back ownership
fn takes_and_gives_back_main() {
    let s1: String = gives_ownership(); //just like the example above, it calls gives_ownership,
                                        //and moved the value from gives_ownership to s1 
    let s2: String = String::from("world"); // declares variable
                                            //
    let s3: String = takes_and_gives_back(s2); // now this is the tricky part, here i'm moving s2
                                               // to takes_and_gives_back, but the function returns
                                               // a string, so it moves the value back to s3, first
                                               // owner is s2, then takes_and_gives_back, making it
                                               // a_string, then moves it to s3, which means we can
                                               // use it in this function
                                               //
    println!("s1 = {}, s3 = {}", s1, s3);   // output should be "s1 = hello, s3 = world"
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
