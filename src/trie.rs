use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::cell::RefCell;

pub struct Trie<K: Eq+Hash+Copy,V> {
    pub value: Option<V>,
    pub children: HashMap<K, RefCell<Trie<K,V>> >,
}

impl<K:Eq+Hash+Copy,V> Trie<K,V> {
    pub fn new() -> Self {
        Trie::<K,V> { value: None, children: HashMap::new() }
    }

    pub fn add<I: Iterator<Item=K>>(&mut self, mut key_set: I, _value: V)
    {
        if let Some(key) = key_set.next() {
            if let Some(child) = self.children.get(&key) {
                child.borrow_mut().add(key_set, _value);
            }
            else {
                self.children.insert(key, RefCell::new(Trie::<K,V>::new()));
                let child = self.children.get(&key).unwrap();

                child.borrow_mut().add(key_set, _value);
            }
        }
        else {
            self.value = Some(_value);
        }
    }
}
