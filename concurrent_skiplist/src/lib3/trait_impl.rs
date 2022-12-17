use crate::IndexOperate;
use crate::lib3::SkipListjjj;

impl<K:Ord,V> IndexOperate<K, V> for SkipListjjj<K,V>{
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        self.inner_get(key,range_end)
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        self.inner_delete(key,range_end)
    }

    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V>{
        self.insert(key,value)
    }

}