extern crate collections;

fn main() {

    let v = collections::vectors::instantiate_vector(45, 32, 27);
    println!("{:?}", v.get(1)); // get returns an Option<T>, which prevents a panic in the case of out-of-bounds or dropped vec<T>.

    let v = collections::vectors::instant_instantiate();
    println!("{}", v[1]); // the v[i] pattern directly returns the vec<T> data, which can cause a panic!

    println!("{}", collections::vectors::enum_vecs());

    collections::strings::string_demo();

    collections::hash_maps::hash_map_demo();
        
}
