pub mod vectors {

    //vectors are a type of common rust collection. they can grow and shrink, are stored on the heap, and take one type of data in. they are written Vec<T>, but pronounced "vector".

    //this is the formal way to create a new vector
    
    pub fn instantiate_vector(a: u32, b: u32, c: u32) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new(); //this is the important line of the function
        v.push(a);
        v.push(b);
        v.push(c);
        v
    }

    // we can, and often do, create a vector instance directly
    
    pub fn instant_instantiate() -> Vec<u32> {
        let v = vec![1, 2, 3];
        v
    }


    pub fn enum_vecs() -> String {
        // bahaha this why enums are good
        // check it out
        //say you want a vec with a buch of different types...


        //set up a vector which contains two instances of an enum, each which has a different type
        struct Type1 {
            data: i32,
            party: bool,
        }

        let t1 = Type1 {data: -420, party: false};

        struct Type2 {
            data: String,
            party: bool,
        }

        let t2 = Type2 {data: String::from("It's time to party"), party: true};
        

        enum Types {
            TypeOne(Type1),
            TypeTwo(Type2),
        }

        let mut v: Vec<Types> = Vec::new();
        v.push(Types::TypeOne(t1));
        v.push(Types::TypeTwo(t2));

        //iterate through the vector, making the different types to their data
        


        for i in &v {
            match i {
                &Types::TypeOne(ref this_struct) => println!("{}", this_struct.data),
                &Types::TypeTwo(ref this_struct) => println!("{}", this_struct.data),
                }
            }


        // a little matching and construction syntax test
        
        let testino = Types::TypeTwo(Type2 {data: String::from("testinorino"), party: true});
        match testino {
            Types::TypeOne(_) => println!("1"),
            Types::TypeTwo(_) => println!("2"),
        }        

        let answer = &v[1];


        match answer {
            &Types::TypeOne(_) => return String::from("You didn't fight for your right to party"),
            &Types::TypeTwo(_) => return String::from("PAAARRRRRTTAAAYY"),
        }
        
        
    }

        
}

pub mod strings {
    // Strings in rust are a collection of bytes, plus a little functionality that lets us interpret the bytes as text
    // in the rust core there's only one type of string: str (which is actually a string slice, and is usually seen as &str
    // string literals are stored in the binary of a program, so it makes sense that we would just slice them to get access
    //String comes from the std library rather than being baked into the language



    pub fn string_demo() {

        // Strings are growable

        let mut s: String = String::from("abcde");
        s.push('f');
        println!("{}", s);

        // you can create Strings like so...

        let s: String = String::new();

        // ... or like so (more likely)

        let s = "initial string value";
        let s: String = s.to_string();
        println!("{}", s);

        // or like so (most likely)

        let mut s: String = String::from("new string");
        println!("{}", &s);

        s.push_str("s can be added to with push_str");
        println!("{}", &s);

        s = s + " or concatenated with the + operator"; // note that this works with a str instead of a String
        println!("{}", &s);

        // we can also use the format! macro to concatenate Strings

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s); // outputs "tic-tac-toe"

        // as we discussed, a String is just a wraper over vec<u8>
        // but you can't index into it like s[4] because various unicode characters have different numbers of bytes (kinda)
        // if you really want to access certain bytes of the string use a range slice like s[16..20]

        // fortunately, the chars() method will do something similar

        let s = String::from("abcdefghij");
        for c in s.chars() {
            println!("{}", c);
        }

        

        
    }
        
    
}

pub mod hash_maps {

    // hash maps bind together values of two different types using a hashing function

    // you have to bring HashMaps into scope
    use std::collections::HashMap;

    pub fn hash_map_demo() {
        // creation
    
        let mut scores = HashMap::new();

        // insertion

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Orange"), 50);

        // hash maps are homogenous: keys have to share one type, and values have to share one type

        // you can also construct hash maps by "collect()"ing a vector of (key, value) tuples

        let teams = vec![String::from("Blue"), String::from("Orange")];
        let start_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(start_scores.iter()).collect();

        // we can get values out of hash map using the get() method, but they come out as Option::Some(value)

        let ref team = String::from("Blue");
        let score = scores.get(team);
        match score {
            Some(s) => println!("the Blue  score is {:?}", s),
            None => println!("There was problem getting the score"),
        }

        

        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
