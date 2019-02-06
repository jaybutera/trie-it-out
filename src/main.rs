mod trie;

use trie::{Trie};

fn main() {
    let mut t = Trie::<char, i32>::new('.', None);
    //t.add("amy", 24);
    //t.add("ann", 30);

    //assert_eq!(t.get("amy"), Some(24));
}
