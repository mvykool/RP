// we define a struct by using the struct keyword, and curly braces, we add the names, and types,
// similar to a class's data definition

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/*

in order to use a struct, we create an instance of that struct by specifying concrete values for each of the fields.
We create an instance by stating the name of the struct, and then add curly brackets containing key value pairs, 
the key are the names of the fields specified in the struct, and the values are the data we want to store in them.
It's like a general template for the type and instances fill in that template with particular data ot create values of the type.
*/

fn main(){
    let user_one = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("username@gmail.com"),
        sign_in_count: 1,
    };
    // we can get specific values from a struct, we use dot notation, like user_one.active. If the
    // instace is mutable, we can even reassign using the dot notation as well like so:
    user_one.active = false;
}
