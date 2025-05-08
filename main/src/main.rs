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


fn app(){               //not valid here, it has not been declared
    let s = "hello";    //s is valid from this point forward
    // -- do something  
}                       //scope ends here, so s is no longer valid
