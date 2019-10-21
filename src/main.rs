use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

fn main(){
    println!("h√k`√i");
    let mut random: HashMap<&str, i32> = HashMap::new();
    random.insert("random", 3);
    let i = random.entry("random");
    match i {
        Occupied(i) => println!("hi"),
        Vacant(i) => println!("hjjjj"),
    }
}