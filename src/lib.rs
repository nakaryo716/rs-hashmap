use std::hash::{DefaultHasher, Hash, Hasher};

const LOAD_FACTOR: f32 = 0.75;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Entry<K, V> {
    key: K,
    val: V,
}

pub struct HashMap<K, V> {
    bucket_size: usize,
    buckets: Option<Vec<Vec<Entry<K, V>>>>,
    size: usize,
}

impl<K: PartialEq + Eq + Hash, V> HashMap<K, V> {
    pub fn new(bucket_size: usize) -> Self {
        let mut buckets = Vec::with_capacity(bucket_size);
        for _i in 0..bucket_size {
            buckets.push(Vec::new());
        }

        Self {
            bucket_size,
            buckets: Some(buckets),
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        // resize
        if self.size as f32 / self.bucket_size as f32 > LOAD_FACTOR {
            self.resize_bucket(self.bucket_size * 2);
        }

        // calc hash by key
        let index = self.calculate_index_by_key(&key);

        // get target bucket
        let bucket = self.buckets.as_mut().unwrap();
        let bucket = bucket.get_mut(index).unwrap();

        // verify key already exist
        let mut same_key_entry = None;
        for entry in bucket.iter_mut() {
            if entry.key == key {
                same_key_entry = Some(entry);
            }
        }
        // replace entry
        if let Some(entry) = same_key_entry {
            *entry = Entry { key, val };
            return;
        }

        // insert value
        bucket.push(Entry { key, val });
        self.size += 1;
    }

    fn resize_bucket(&mut self, new_bucket_size: usize) {
        // update bucket size
        self.bucket_size = new_bucket_size;
        // allocation new buckets
        let mut new_buckets = Vec::with_capacity(new_bucket_size);
        for _i in 0..new_bucket_size {
            new_buckets.push(Vec::new());
        }

        // reinsert
        for bucket in self.buckets.take().unwrap() {
            for entry in bucket {
                let index = self.calculate_index_by_key(&entry.key);

                let target_bucket = new_buckets.get_mut(index).unwrap();
                target_bucket.push(entry);
            }
        }

        self.buckets = Some(new_buckets);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        // calc hash
        let index = self.calculate_index_by_key(key);

        // get bucket
        let bucket = self.buckets.as_ref().unwrap();
        let bucket = bucket.get(index).unwrap();

        // find value by key
        bucket.iter().find(|v| v.key == *key).map(|v| &v.val)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.calculate_index_by_key(key);
        let bucket = self.buckets.as_mut().unwrap();
        let bucket = bucket.get_mut(index).unwrap();

        // FIXME:
        // bad implementation for performance
        //
        // search index that match key from a bucket
        let mut bucket_idx = None;
        for (i, val) in bucket.iter().enumerate() {
            if val.key == *key {
                bucket_idx = Some(i);
            }
        }
        // remove from bucket and swap elements
        bucket_idx.map(|v| bucket.remove(v)).map(|entry| {
            self.size -= 1;
            entry.val
        })
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn calculate_index_by_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        hash as usize % self.bucket_size
    }
}

#[cfg(test)]
mod tests {
    use crate::{Entry, HashMap};

    #[test]
    fn test_map() {
        let mut a = HashMap::<String, String>::new(10);

        if a.buckets.as_mut().unwrap().get_mut(0).is_none() {
            a.buckets.as_mut().unwrap().push(Vec::new());
        }

        let val = a.buckets.as_mut().unwrap().get_mut(0).unwrap();

        val.push(Entry {
            key: "hello".into(),
            val: "world".into(),
        });

        let val = &a.buckets.as_mut().unwrap()[0].pop().unwrap();

        assert_eq!(val.key, "hello");
        assert_eq!(val.val, "world");
    }

    #[test]
    fn insert_and_get() {
        let mut map = HashMap::new(32);
        map.insert("hello", "world");

        assert_eq!(map.get(&"hello"), Some(&"world"));
        assert_eq!(map.get(&"none"), None);
    }

    #[test]
    fn insert_modify_get() {
        let mut map = HashMap::new(32);

        map.insert("hello", "world");
        map.insert("hello", "world2");

        assert_eq!(map.get(&"hello"), Some(&"world2"));
    }

    #[test]
    fn remove() {
        let mut map = HashMap::new(32);
        map.insert("hello", "world");

        assert_eq!(map.remove(&"hello"), Some("world"));
        assert_eq!(map.remove(&"hello"), None);
        assert_eq!(map.get(&"hello"), None);
    }

    #[test]
    fn len() {
        let mut map = HashMap::new(32);
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());

        map.insert("hello", "world");
        assert_eq!(map.len(), 1);
        map.insert("hello2", "world2");
        assert_eq!(map.len(), 2);
        assert!(!map.is_empty());

        map.remove(&"hello");
        assert_eq!(map.len(), 1);
        map.remove(&"hello2");
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn resize() {
        let mut map = HashMap::new(1);

        map.insert("hello0", "world0");
        map.insert("hello1", "world1");
        map.insert("hello2", "world2");
        map.insert("hello3", "world3");
        map.insert("hello4", "world4");

        assert_eq!(map.get(&"hello0"), Some(&"world0"));
        assert_eq!(map.get(&"hello1"), Some(&"world1"));
        assert_eq!(map.get(&"hello2"), Some(&"world2"));
        assert_eq!(map.get(&"hello3"), Some(&"world3"));
        assert_eq!(map.get(&"hello4"), Some(&"world4"));
    }
}
