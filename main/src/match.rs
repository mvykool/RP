// we will create a small program that will multiple by 5 any given input
fn main() {
    let five: Option<i32> = Some(5);
    let value: Option<i32> = times_five(five);
    let none: Option<i32> = times_five(None);
}

// lets create a function that will take an optional enum parameter, and multiply it by 5
// as long as there's a value of course
fn times_five(number: Optional<i32>) -> Optional<i32> {
    // here lets use a match statement, and handle the possible outcomes
    match number {
        // if its none, we return none
        None => None,
        // if the value exist, we multiply by 5
        Some(i: i32) => Some(i * 5), // notice how we cant do i * 5, we have to do it inside Some
    }
    // remember when using a match expression we have to handle all possible variants
    // however, if we just want to define one, we can use an underscore
    //
    // in this case we say, if number is some, execute somethig, otherwise, its none
    // in any other case that isnt some, its none
    match number {
        Some(5) => println!("five"),
        _ => None,
    }
}


// if let
//
// another way to handle things, if we only care about one variant, if let its csomething we can
// use
//
//matching expression example
if example() {
    let some_value: Option<i32> = Some(3);
    
    match some_value {
        Some(3) => println!("Three!");
        _ => (), // this means none
    }
}

//if let example
if example() {
    let some_value: Option<i32> = Some(3);
    
    // if let, some_value is 3, then print "three!", this way we only care about one variant
    // it seems consfusion, as if some and some_value are reversed
    if let Some(3) = some_value {
        println!("three!");
    }
}
