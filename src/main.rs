mod trie;

use trie::{Trie};

fn main() {
    let mut t = Trie::<char,i32>::new();
    t.add("amy".chars(), 24);
    //t.add("ann".chars(), 30);

    //assert_eq!(t.get("amy"), Some(24));
}
