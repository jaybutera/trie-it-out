mod trie;

use trie::{Trie};

fn main() {
    let mut t = Trie::<char,i32>::new();
    t.add("amy".chars(), 24);
    match t.get("amy".chars()) {
        Some(_) => println!("found!"),
        None => println!("not there"),
    }
    //t.add("ann".chars(), 30);

    //assert_eq!(t.get("amy"), Some(24));
}
