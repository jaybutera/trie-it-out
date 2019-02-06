pub struct Trie<K,V> {
    pub key:      K,
    pub value:    Option<V>,
    pub next:     Option< Box<Trie<K,V>> >,
    pub children: Option< Box<Trie<K,V>> >,
}

impl<K,V> Trie<K,V> {
    pub fn new(_key: K, _value: Option<V>) -> Self {
        Trie {
            key:      _key,
            value:    _value,
            next:     None,
            children: None,
        }
    }
}
