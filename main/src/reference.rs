// now we are going to do a borrowing example
// instead of passing the value of something
// we can simply pass their reference, that way we don't have to take, and give values
// we can pass a reference, use it, references are by default inmutable
// but we can make them mutable by using "mut"
//

fn main(){
    let s1: String = String::from("hello");
    let len: unsize = calculate_length(&s1); //now here instead of passing "s1" we add the & sing to
                                          //it like this "&s1", now we are passing the reference,
                                          //not the actual variable value 
    println!("the length of '{}', is {} ",s1, len );
}

fn calculate_length(s: &String) -> unsize { //for the type String, we also pass the reference, we
                                            //borrowing the value, not owning it
    let length: unsize = s.len();
    length
}


// as mentioned references are by definition inmutable, but what if we want to mutate it??
// in that case we pass a mutable reference, and make the function take a mutable reference

fn main_mutable(){
    let mut s1: String = String::from("hello"); // first we have to make the variable muttable
    change_string(&mut s1); // this is how we pass a mutable variable
}

fn change_string(string: &mut String){ //this is how we pass a mutable variable 
    string.push_str(", world"); //now we can modify its value
}
