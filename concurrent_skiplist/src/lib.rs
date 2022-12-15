mod node;

pub struct ConcurrentSkiplist<K:Ord,V>{
    k:K,
    v:V
}

impl<K:Ord,V> ConcurrentSkiplist<K,V> {
    pub fn new(){

    }
}
/// Operations of Index
/// trait with generic type
pub(crate) trait IndexOperate<K: Ord, V> {
    /// Get a range of keys in [key, range_end]
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    /// delete a range of keys in [key, range_end]
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}

impl<K:Ord,V> IndexOperate<K, V> for ConcurrentSkiplist<K,V>{
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>{

        Vec::<&V>::new()
    }
    /// delete a range of keys in [key, range_end]
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>{
        Vec::<V>::new()
    }
    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V>{
        None
    }
}