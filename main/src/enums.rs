// enum is short for enumeration, a good way to grouping together related fields and data, like a
// rectangle, with its width and height, enums give you a way of saying a value is one of a
// possible set of values. A rectangle is one of a set of possible shapes, that also includes
// circles, squares, triangles

enum IpAddrKind { // this is now a custom data type we can use anywhere
    // we can store data in the enum variants
    V4(u8, u8, u8, u8),
    V6(String),
}

// enums can store a wide variety of types
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can create instances of each of the variants like so
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// note that variants of the enum are namespaced under its undetifier, and we use a double colon to
// separate the two. This is useful because now both values are the same type,. We can then, for
// instance,define a function that takes any IpAddrKind
fn route(ip_kind: IpAddrKind){}

//now we can call this function with either variant:
route(IpAddrKind::V4);
route(IpAddrKind::V6);

fn main(){
    let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
}


// OPTION ENUM
//
// a value could exist or be null, in Rust there are non null values
enum Option<T> {
    //it only has 2 values
    some(T), //generic, it could be any value
    None,
}
// we have a value that could potentially be null or non-existing, we can wrap it in a option enum,
// this will force the type system to enforce that we handle none cases
//
// examples
fn main() {
    let some number: Option<i32> = Some(5);
    let some string: Option<&str> = Some("a string");

    let absent number: Option<i32> = None;
}

// using option enum
fn main(){
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // optional integer, it could be an integer or nothing, let's try
                                 // to add x and y
    
    let sum = x + y;// we get a compile error, we cannot add an optinal type to an integer, we need
                    // to extract out of the Some variant, we need to write code that handles all
                    // posible variants, example if the variant is some, we can use the value, but
                    // if it's none, we need to define what the program should do
                    //
    let sum = x + y.unwrap_or(default: 0); //if value exist, use it, otherwise, use default value,
                                           //which is 0
}


// MATCH EXPRESSION
//
// it allows us to compare an expression against a set of patterns, these patterns could be
// literal, variables, etc
//
// having expresive patterns make MATC statement powerful, along with the fact that the match
// expression is exhaustive, we gotta match ALL possible cases, this makes it useful for enums
//

enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // we return the value of every variant using match expression
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // here we are matching on the quater variant, if the code is short, we dont need curly
        // brackets, in this case, we bind the state to this quater, and print the value
        Coin::Quarter(state: UsState) => {
            println!("State quarter from: {:?}!", state);
            25
        },
    }
}


// this is how you would call the Quater from the coin enum
fn main(){
    value_in_cents(Coin::Quater(UsState::Alaska));
}


