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
