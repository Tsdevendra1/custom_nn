use std::collections::HashMap;

fn main(){
    let mut layer_nodes : HashMap<&i32, Vec<&i32>> = HashMap::new();
    layer_nodes.insert(&1, vec![&2]);

    let i  = &layer_nodes;
    let j  = i.get(&1).unwrap();
    println!("h√k`√i");
}