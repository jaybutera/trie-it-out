use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::cell::RefCell;

pub struct Trie<K: Eq+Hash+Copy,V> {
    pub value: Option<V>,
    pub children: HashMap<K, Box<Trie<K,V>> >,
}

impl<K:Eq+Hash+Copy,V:Copy> Trie<K,V> {
    pub fn new() -> Self {
        Trie::<K,V> { value: None, children: HashMap::new() }
    }

    pub fn add<I: Iterator<Item=K>>(&mut self, mut key_set: I, _value: V)
    {
        if let Some(key) = key_set.next() {
            if let Some(mut child) = self.children.remove(&key) {
                child.add(key_set, _value);
                self.children.insert(key, child);
            }
            else {
                let mut child = Box::new(Trie::<K,V>::new());
                child.add(key_set, _value);
                self.children.insert(key, child);
            }
        }
        else {
            self.value = Some(_value);
        }
    }

    pub fn get<I: Iterator<Item=K>>(&self, mut key_set: I) -> Option<V> {
        match key_set.next() {
            Some(key) => {
                match self.children.get(&key) {
                    Some(child) => child.get(key_set),
                    None => None, 
                }
            },
            None => self.value,
        }
    }
}
