// some data types in rust are scalar, mainly booleans, and integers, foats, numbers, bolleans, and
// charaters 
//
//tuple type

fn main(){
    //a tuple is a general way of grouping together a number of values with a variety of types into
    //one compound type. Tupes have fixed length, once declared, they can't grow nor shrink in size
    //we create a tuple by writting a comma-separated list of values, inside parentheses.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // the variable tup. binds to the entire tuple because a tuple is considered a single compound
    // element. To get the individual values out of a tuple we can use pattern matching to
    // destructure a tuple value. like so:
    //
    let (x, y, z) = tup;

    println!("the value of y, is: {y}");

    //we can also access a tuple element directly by usnig a period, followed by the index of the
    //value we want to access, Example:
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

//Array type
//
/*
 another way to have a collection fo multiple values is with an array. Unlike a tuple, every element of  an array
must have the same type. Unlike arrays in some other langages, in rust they have a fixed length

we write the values in an array as a comma-separated list inside square brackets
 */

fn main() {
    let a = [1, 2, 3, 4, 5];

    // you write an array's type using square brackets with the type of each element, a semicolon
    // and then the number of elements in the array like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //to access values, we do so like this:
    let first = a[0];
    let last = a[4];
}
