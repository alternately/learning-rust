fn main() {
    // structs are like tuples, but with named elements
    // they apparently are used for making types?

    // here's an example struct, made to hold user data

    struct User { //note the capital letter
        username: String,  //name, type pairs (Obviously)
        email: String,
        sign_in_count: u64,
        active: bool,
    }  // no semicolon!

    // you can create instances of structs by providing data for all the fields

    let alternately = User {
        active: true,
        email: String::from("apgiuffre@gmail.com"),
        sign_in_count: 1337,
        username: String::from("alternately"),
    };

    //data comes out from structs like you would expect

    println!("username: {}", alternately.username);
    println!("email: {}", alternately.email);
    println!("sign in count: {}", alternately.sign_in_count);
    println!("active?: {}", alternately.active);

    // you can make mutable structs

    let mut alternately = User {
        active: false,
        email: String::from("apgiuffre@gmail.com"),
        sign_in_count: 1336,
        username: String::from("alternately"),
    };

    // and can edit mutable struct contents easily

    alternately.active = true;
    alternately.sign_in_count = alternately.sign_in_count + 1;

    println!("username: {}", alternately.username);
    println!("email: {}", alternately.email);
    println!("sign in count: {}", alternately.sign_in_count);
    println!("active?: {}", alternately.active);

    // when initializing structs, if variables have the same name as struct fields, you can use this shorthand:

    let email: String = String::from("apgiuffre@example.net");
    let username: String = String::from("apgismyusername");

    let apg = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };


    // struct update syntax is another shortcut used to create slightly edited versions of preexisting structs

    let apg2 = User {
        username: String::from("myusernameisapg"), //manually set a username
        ..apg, //everything else as in apg
    };

    // we can create tuple structs, which behave like tuples, but each have their own type

    struct Example1(i32, String, bool, bool, u32);
    struct Example2(i32, String, bool, bool, u32);
    //these two AREN'T interchangeable, and have different types

    //there will be more on structs and it will go here |
    //                                                  |
    //                                                  V


    // I'm on chapter 11, and no more on structs really
    
    
    
    
}
