fn main() {
    let x = 3;

    // standard if/else syntax as follows...

    
    
    if x < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }


    if x != 2 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    //else if
    
    if x == 2 {
        println!("x = 2");
    } else if x == 3 {
        println!("x = 3");
    } else if x == 3 {
        println!("lazy evaluation means this won't show up!");
    } else {
        println!("x is neither 2 nor 3");
    }
    
}
