use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::hash::Hash;

trait Storage<K, V> {
    fn get(&self, key: &K) -> Option<V>;
    fn delete(&mut self, key: &K) -> bool;
    fn insert(&mut self, key: K, value: V) -> bool;
}

struct InMemoryStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Storage<K, V> for InMemoryStorage<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        self.data.get(key).cloned()
    }

    fn insert(&mut self, key: K, value: V) -> bool {
        self.data.insert(key, value).is_none()
    }

    fn delete(&mut self, key: &K) -> bool {
        self.data.remove(key).is_some()
    }
}

struct KeyValueStore<K, V> {
    storage: Box<dyn Storage<K, V>>,
}

impl<K, V> KeyValueStore<K, V>
where
    K: Eq + Hash + 'static,
    V: Clone + 'static,
{
    fn new() -> Self {
        KeyValueStore {
            storage: Box::new(InMemoryStorage {
                data: HashMap::new(),
            }),
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        self.storage.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> bool {
        self.storage.insert(key, value)
    }

    fn delete(&mut self, key: &K) -> bool {
        self.storage.delete(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut store = KeyValueStore::new();
        store.insert(1, "Rust".to_string());
        assert_eq!(store.get(&1), Some("Rust".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut store = KeyValueStore::new();
        store.insert(1, "Rust".to_string());
        assert_eq!(store.delete(&1), true);
        assert_eq!(store.get(&1), None);
    }

    #[test]
    fn test_non_existent_key() {
        let store: KeyValueStore<i32, i32> = KeyValueStore::new();
        assert_eq!(store.get(&2), None);
    }
}
