// methods are just like functions, we pass parameters, we return values, they contain some code
// that's run when the method is called from somewhere else. Unlike functions, methods are defined
// within the context of a struct, or an enum, or  trait object
// their first parameter is ALWAYS self, which represents the instace of the struct the method is
// being called on
//
// we use impl 
// impl is how you attach behavior to types in Rust — it's a core part of the language's approach to object-oriented and trait-based programming.

struct Rectangle {
    width: u32,
    height: u32,
}

// instead of creating the method inside of a object as we would do in JS, here we create an
// implementation, extending the struct, and adding a method to it, which is just a function inside
// of a struct lol, always using self, which is the representation of the instance, kind of like
// `this` in JS
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// creating associated functions, they are associated with the tupe named after the impl
// we can define associated functions that don't have self as their first parameter (thus theyre no
// methods), becayse they dont need an instance of the type to work with.
//
// functions that arent methods are often used for constructors that will return a new instance of
// the struct.
//
impl Rectangle {
    fn square(size: u32) -> Self {
        Self{
            width: size,
            height: size
        }
    }
}

// Self keyword in the return type and in the body of the functions are alliases for the type that
// appears after the impl keyword, which in this case is Rectangle
//
// to call this associated function we use the :: syntax with the struct name like this:
// `let sq = Rectangle::square(3);`
