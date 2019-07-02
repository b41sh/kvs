use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new KvStore.
    pub fn new() -> KvStore {
        KvStore { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        //self.map.get(key)
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        //self.map.remove(key)
        self.map.remove(&key);
    }
}
