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

// we can construct instances of the struct as the last example in the function body to implicity
// return that new instance

//this is a function that returns a User struct instance, with a given email, and user name

fn build_user (email: String, username: String ) -> {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }

    // in cases where the names of the fields, and the name of the parameters are the same we can
    // use field init shorthand, cool to avoid repetition
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Create a new instance from other instaces with struct update syntax

fn two_instances(){

    //this is the same struct instance we created earlier
    let user_one = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("username@gmail.com"),
        sign_in_count: 1,
    };

    // this is creating instace from another instance, struct update syntax
    let user_two = User {
        active:  user_one.active,
        username: user_one.username,
        email: String::from("usertwo@gmail.com"),
        sign_in_count: user_one.sign_in_count,
    }

    // here is a cleaner way to do the same thing, its like spread operator in JS
    // only modify the fields we want to change and "copy" the rest
    let user_two = User {
        email: String::from("usertwo@gmail.com"),
        ...user_one,
    };
}


