use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

#[derive(Debug, Clone)]
pub(crate) struct StackedHashMap<K, V, S = RandomState> {
    map_stack: Vec<HashMap<K, V, S>>,
}

impl<K, V, S> StackedHashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub(crate) fn new() -> Self {
        StackedHashMap { map_stack: vec![] }
    }

    pub(crate) fn from(m: HashMap<K, V, S>) -> Self {
        StackedHashMap { map_stack: vec![m] }
    }

    pub(crate) fn push(&mut self, m: HashMap<K, V, S>) {
        self.map_stack.push(m);
    }

    pub(crate) fn pop(&mut self) -> Option<HashMap<K, V, S>> {
        self.map_stack.pop()
    }

    pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
        let last_index = self.map_stack.len() - 1;
        self.map_stack[last_index].insert(k, v)
    }

    pub(crate) fn get(&self, k: &K) -> Option<&V> {
        for env in self.map_stack.iter().rev() {
            if let Some(t) = env.get(k) {
                return Some(t);
            }
        }
        None
    }

    pub(crate) fn stack_is_empty(&self) -> bool {
        self.map_stack.is_empty()
    }
}

impl<K, V, S> StackedHashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher + Default,
{
    pub(crate) fn into_map(self) -> HashMap<K, V, S> {
        self.map_stack.into_iter().flatten().collect()
    }
}

impl<K, V, S> Default for StackedHashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, S> StackedHashMap<K, V, S>
where
    K: Hash + Eq,
    V: Default,
    S: BuildHasher + Default,
{
    pub(crate) fn push_default(&mut self) {
        self.push(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::stacked_hash_map::StackedHashMap;
    use std::collections::HashMap;

    #[test]
    fn test_into_map() {
        let mut smap = StackedHashMap::new();
        smap.push(HashMap::new());
        smap.insert("1", 1);
        smap.insert("2", 2);
        smap.push(HashMap::new());
        smap.insert("2", 4);

        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 4);
        assert_eq!(smap.into_map(), map);
    }

    #[test]
    fn test_stack_is_empty() {
        let mut smap: StackedHashMap<&str, &str> = StackedHashMap::new();
        assert!(smap.stack_is_empty());
        smap.push(HashMap::new());
        assert!(!smap.stack_is_empty());
    }
}
