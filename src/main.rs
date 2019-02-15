mod trie;

use trie::{Trie};

fn byte_string(x: i32) -> String {
    let mut v = x;
    let mut bytes = String::from("");

    while v >= 1 {
        bytes.push(if v % 2 == 1 {'1'} else {'0'});
        v = v / 2;
    }

    bytes
}

fn main() {
    let mut t = Trie::<char,i32>::new();

    for i in 0..100000 {
        t.add(byte_string(i).chars(), 24);
    }
    //assert_eq!(t.get("amy".chars()), Some(24));
}
