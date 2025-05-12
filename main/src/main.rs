use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main(){
    let stdout = stdout();
    let message = String::from("hello fellow Rustacean");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

//ownership understanding

// RULES TO LIVE BY
// * each value in Rust has an owner
// * there can be only one owner
// * once the owner goes out of scope, the value will be dropped

// ownership examples

//way to manage memory 
// other solutions: 
// garbage collector
// memory management model: memory allocation
// ownership unique rust way of doing things

fn ownership(){
    fn a(){
        let x: &str = 'hello';
        let y: i32 = 22;
        b();
    }

    fn b(){
        let x: String = String::from("World")
    }
}


fn app(){               //not valid here, it has not been declared
    let s = "hello";    //s is valid from this point forward
    // -- do something  
}                       //scope ends here, so s is no longer valid

let mut string = String::from("hello");

string.push_str(", world"); //push_str() appnds a literal to a string

println!("{string}") //this will output hello, world

