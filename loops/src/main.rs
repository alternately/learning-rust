fn main() {
    // rust has three kinds of loops...
    // ... loop.

    let mut i: u8 = 0;
    
    loop {
        println!("i: {}", i);
        i = i + 1;
        if i > 7 {
            break;
        }
    }

    // ... while

    while i < 16 {
        println!("i: {}", i);
        i = i + 1;
    }

    // ... and for

    let ai = [16, 17, 18, 19, 20, 21, 22, 23];

    for element in ai.iter() {
        println!("!i: {}", element);
    }

    // there is a general preference for for loops
    // even iteration of code over a certain number of repetitions generally should use for with a Range collection

    for j in (24..32) { // (24..32) indicates a "range"
        println!("!i: {}", j);
    }
    
}
