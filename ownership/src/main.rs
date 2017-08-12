fn main() {

    //ownership is how rust manages memory on the heap
    //it follows three rules:

    //1. All values in rust have a variable called the values owner
    //2. there can be one owner
    //3. when the owner goes out of scope, the value is dropped


    let s = "I'm a string literal"; // immutable string literals are hardcoded in at compiletime

    // String::from() makes a String (mutable heapy data type) out of a string literal (immutable).
    
    let mut s = String::from("hello");

    s.push_str(", world.");
    println!("{}", s);


    // when rust is dealing with primative data on the stack, it doesn't use ownership
    let x: i32 = 16; //rust knows to allocate exactly 32 bits to the stack
    let y = x; //again, rust knows exactly what to put on the stack. No problem.
    println!("x: {}", x);
    println!("y: {}", y);


    //with heapy, stretchy datatypes, however, rust needs to use ownership rules.
    //for example, String...

    let s = String::from("on the heap!");
    let s2 = s;  // this is doing a "move", like a shallow copy in that it only copys a pointer to heap data, but it also invalidates s to prevent double dropping.
    //  println!("s: {}", s); doesn't work because s no longer has ownership of the data!!
    println!("s2: {}", s2);

    //rust never deep-copies data by default (deep copying is taking all the heap data and copying it, and providing a new pointer to the clone data on the stack. shallow copying leaves the heap data untouched, but provides a copy of the pointer to it.

    // in order to deep copy in rust, use clone
    //but be wary, deep copy can be expensive.

    let s = String::from("on the heap!");
    let s2 = s.clone();
    println!("s: {}", s);
    println!("s2: {}", s2);

    
}
