use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::cell::RefCell;

pub struct Trie<K: Eq+Hash+Copy,V> {
    pub value: Option<V>,
    pub children: HashMap<K, *mut Box<Trie<K,V>> >,
}

impl<K:Eq+Hash+Copy,V:Copy> Trie<K,V> {
    pub fn new() -> Self {
        Trie::<K,V> { value: None, children: HashMap::new() }
    }

    pub fn add<I: Iterator<Item=K>>(&mut self, mut key_set: I, _value: V)
    {
        if let Some(key) = key_set.next() {
            if let Some(mut raw_child) = self.children.get(&key) {
                unsafe {
                    (**raw_child).add(key_set, _value);
                }
            }
            else {
                self.children.insert(key, (&mut Box::new(Trie::<K,V>::new())));
                let raw_child = self.children.get(&key).unwrap();

                unsafe {
                    (**raw_child).add(key_set, _value);
                }
            }
        }
        else {
            self.value = Some(_value);
        }
    }

    pub fn get<I: Iterator<Item=K>>(&self, mut key_set: I) -> Option<V> {
        match key_set.next() {
            some(key) => {
                match self.children.get(&key) {
                    some(raw_child) => unsafe { (**raw_child).get(key_set) },
                    None => None, 
                }
            },
            None => self.value,
        }
    }
}
