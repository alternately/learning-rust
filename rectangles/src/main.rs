// on the surface, this program computes the area of rectangles
// but it's really practice for applying structs to everyday code

fn main() {
    one();
    two();
    three();
    four();
    five();
}


//version one doesn't use structs

fn one() {
    let length1 = 50;
    let length2 = 30;

    println!("the area of this rectangle is {} square pixels", areaone(length1, length2));
}

fn areaone(length: i32, width: i32) -> i32 {
    length * width
}

// version two uses a tuple to hold the dimension data of the rectangle

fn two() {
    let dimensions = (50, 30);

    println!("the area of this rectangle is {} square pixels", areatwo(dimensions));
}

fn areatwo(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

// version 3 uses structs to hold the dimensions of the rectangle

struct Rectangle {
    length: u32,
    width: u32,
}

fn three() {
    let rekt = Rectangle {length: 50, width: 30};
    println!("the area of this rectangle is {} square pixels", areathree(rekt));

}

fn areathree(rect1: Rectangle) -> u32 {
    rect1.length * rect1.width
}


//while we're at it, function four shows how to use alternate formatters ({} is the default formatter) to display structs

#[derive(Debug)]
struct DebugRectangle {
    length: u32,
    width: u32,
}

fn four() {
    let examplino = DebugRectangle {length: 42, width: 69};
    println!("examplino looks like this: {:#?}", examplino);
}


// finally, we can *implement* methods in structs, which are functions bound to a struct

struct MethRectangle {
    length: u64,
    width: u64,
}

impl MethRectangle {
    fn areafive(&self) -> u64 { //the first argument of a method is always a reference to "self", which will reference the specific instance of the struct calling the method
        self.width * self.length
    }

    fn can_hold(&self, other: &MethRectangle) -> bool {
        if self.length > other.length && self.width > other.width {
            return true;
        } else {
            return false;
        }
    }
    
} // easy!

fn five() {
    let finalrect = MethRectangle {length: 420, width: 8008135};
    let psrect = MethRectangle {length: 550, width: 87};
    println!("the area of this rectangle is {}", finalrect.areafive());
    println!("the statement 'finalrect can hold psrect within it is {}", finalrect.can_hold(&psrect));
}


//structs are a variety of custom type, but they aren't the only one?
