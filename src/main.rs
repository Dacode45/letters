use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

/// Every lowercase letter
static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];


/// Initializes hashmap with false for every letter
fn init(alphabet: &mut HashMap<char, bool>) {
    for c in ASCII_LOWER.iter() {
        alphabet.insert(c.clone(), false);
    }
}

/// Outputs letters used by an input
fn main() {
    let mut alphabet: HashMap<char, bool> = HashMap::new();
    init(&mut alphabet);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("ERROR");
    for c in input.to_lowercase().chars() {
        alphabet.insert(c.clone(), true);
    }
    let mut used = Vec::new();
    let mut ignored = Vec::new();

    for c in ASCII_LOWER.iter() {
        if *alphabet.get(c).unwrap() {
            used.push(c)
        } else {
            ignored.push(c)
        }
    }
    println!("{:?}\n{:?}", used, ignored);
}
