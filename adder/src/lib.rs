#[cfg(test)]


//utility module for demoing rust's testing functionality
pub mod rectangles {

    #[derive(Debug)]
    pub struct Rectangle {
        pub length: u32,
        pub width: u32,
    }

    impl Rectangle {

        pub fn can_contain(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }
}

mod tests {

    use super::rectangles::*;
    
    #[test]
    fn it_works() {
    }

    // test fail when they cause a panic
    #[test]
    fn it_doesnt_work() {
        panic!("make this test fail");
    }

    #[test]
    fn rectangle_test() {
        let r1 = rectangles::Rectangle {length: 10, width: 10};
        let r2 = rectangles::Rectangle {length: 8, width: 8};

        //the assert! macro lets us test the truth value of a boolean (kinda like clojure's "is" testing construct)
        assert!(r1.can_contain(&r2));

        let r3 = rectangles::Rectangle {length: 10, width: 10};
        let r4 = rectangles::Rectangle {length: 10, width: 11};

        assert!(!r3.can_contain(&r4));
    }
}
