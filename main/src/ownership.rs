fn main(){
    let s: String = String::from("Hello"); // we declare a variable, type string, which is dynamic,
                                           // and stored in heap
                                           //
    takes_ownership(s);                    // we pass the s variable to this function as a
                                           // parameter, however, keep in mind that passing 
                                           // a variable as a parameter, if its type dynamic its
                                           // the same as if we were moving it to a different
                                           // variable

    println!("{}", s)                     // therefore, this will not run, because the s value
                                          // moved to the function as parameter
}

fn takes_ownership(string_parameter: String) { // since we are moving s variable to this function,
                                               // its now string_parameter variable, once the function
                                               // scope ends, string_parameter is dropped
    println!("{}", string_parameter);
}
