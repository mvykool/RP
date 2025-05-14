// A slice is a view into a part of a collection (like an array, vector, or string) — without owning the data.
//Think of a slice like saying:
// “I don’t want the whole cake — just a few slices from it. And I don’t need to own the cake, just eat from it.”

let arr: [i32; 5]  = [10, 20, 30, 40, 50];

let slice: &[i32] = &arr[1..4];
// the output of slices would be [20, 30, 40]
// arr has type [i32; 5]
// slice has type &[i32] — a reference to a slice of integers
//
let arr: [i32; 4] = [5, 6, 7, 8];

let full_slice: &[i32] = &arr[..]; 
// full_slice has type &[i32], referencing the whole array

// strings
let s: String = String::from("hello world");

let hello: &str = &s[0..5]; 
// hello = "hello"
// hello is a &str (string slice)


