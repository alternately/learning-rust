fn main() {
    // enums are a kind of type where all possible values have been enummerated ahead of time
    // camel case is appropriate for naming

    //you can have exactly two different kinds of ip address, and here they are
    
    enum IpAddrKind {
        V4,
        V6,
    }

    // we can instantiate enums like so:

    let v4 = IpAddrKind::V4;  // variants are namespaced under the enum indentifier
    let v6 = IpAddrKind::V6;

    // both values have the same type, so we can create a funtion to use either

    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => {println!("heya");},
            IpAddrKind::V6 => {println!("whasup");},
        }
    }

    route(v4);
    route(v6);


    // you can store data in enums, however

    enum IpAddr {
        V4(String), // (String means that this outcome has an associated String value
        V6(String),
    }

    // we instantiate normally

    let ip_address = IpAddr::V4(String::from("127.0.0.1"));

    // enums are cool because they can do this (among other reasons)

    enum VarietyPack {
        Nuthin,
        Color(u32, u32, u32, u32),
        Signature(String),
        Anon { a: i32, b: String } //this is binding an enumerated result to an anonymous structure
    }

    // enums can also implement methods

    impl VarietyPack {
        fn call(self) {
            match self {
                VarietyPack::Nuthin => {},
                VarietyPack::Color(r, g, b, a) => println!("r: {} g: {} b: {} a: {}", r, g, b, a),
                VarietyPack::Signature(sig) => println!("your signature is {}", sig),
                VarietyPack::Anon{a, b} => println!("a: {} b: {}", a, b),
            }
        }
    }

    //such methods are called like so

    let v = VarietyPack::Signature(String::from("YO10sw4g"));
    v.call();


    // Option<T> is a standard enum in rust
    // it comes in the prelude so we don't need to extern it or even namespace its variants
    // the variants are Some<T>, and None
    // the <T> stands for any type
    // Option::None is the closest rust comes to having null or nil
    // we use option a lot for error checking, etc
    // this sort of deal is apparently common


    fn plus_one(opt: Option<f64>) -> Option<f64>{
        match opt {
            None => None,
            Some(i) => Some(i + 1.0),
        }
    }

    let six: Option<f64> = plus_one(Some(5.0));


    // match is exhaustive, you have to provide a branch for every possible option
    
    match &six {
        &None => println!("something went wrong..."),
        &Some(i) => println!("5 + 1 = {}", i),
    }

    // but you can use the _ place holder to stand in for "all the other options"

    enum LotsOfOpts {
        A,
        B,
        C,
        D,
        E,
    }

    let this_opt = LotsOfOpts::C;

    match this_opt {
        C => println!("you chose option C"),
        _ => println!("you chose an option other than C"),
    }

    // to be more concise, we can also use "if let"
    // this only works if we're only checking for exactly one possibility
    // this is weird syntax sugar, don't worry too much about it

    let that_opt = LotsOfOpts::D;

    if let LotsOfOpts::D = that_opt {
        println!("you da best");
    }
        
}
