use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Entry<K, V> {
    key: K,
    val: V,
}

pub struct HashMap<K, V> {
    bucket_size: usize,
    buckets: Vec<Vec<Entry<K, V>>>,
    size: usize,
}

impl<K: Hash, V> HashMap<K, V> {
    pub fn new(bucket_size: usize) -> Self {
        Self {
            bucket_size: bucket_size,
            buckets: Vec::with_capacity(bucket_size),
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        // resize

        // calc hash by key

        // bucket create if dose not exist

        // verify key already exist

        // insert value
        todo!()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        // calc hash

        // get bucket

        // find value by key
        todo!()
    }

    pub fn calculate_index_by_key(&self, key: K) -> usize {
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

        if a.buckets.get_mut(0).is_none() {
            a.buckets.push(Vec::new());
        }

        let val = a.buckets.get_mut(0).unwrap();

        val.push(Entry {
            key: "hello".into(),
            val: "world".into(),
        });

        let val = &a.buckets[0].pop().unwrap();

        assert_eq!(val.key, "hello");
        assert_eq!(val.val, "world");
    }
}
