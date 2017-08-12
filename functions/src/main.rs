fn main() {
    println!("Hello, world!");
    new_function();
    param_func(7, 'c');
    println!("s was set to: {}", statement_expression());
}

// functions are declared like so, and can happen before or after where they're called.

fn new_function() {
    println!("this text provided courtesy of new_function");
}


// functions take paramaters

fn param_func(x: i32, y: char) {
    println!("the parameter x was given the argument {}", x);
    println!("the parameter y was given the argument {}", y);
}


//a function body is a series of statements optionally followed by an expression
//statements and expressions are different things.

//statements don't return anything

fn statement_expression() -> i32 { // -> i32 declares a return type. Mandatory?
    //statement
    let s = 0;
    //expression, note the lack of end semicolon
    s
}
