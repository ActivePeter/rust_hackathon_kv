extern crate core;

use core::panicking::panic;

mod node;
const MAX_HEIGHT:i32 =12;
const K_BRANCHING:usize=4;
pub struct ConcurrentSkiplist<K:Ord,V>{
    k:K,
    v:V
}

impl<K:Ord,V> ConcurrentSkiplist<K,V> {
    pub fn new(){

    }
    fn random_height(&self)->i32{
        let mut height:i32 = 1;
        while height < MAX_HEIGHT && rnd_.OneIn(K_BRANCHING) {
            height+=1;
        }
        assert!(height > 0);
        assert!(height <= kMaxHeight);
        return height;

    }
}
//     SkipList<Key, Comparator>::FindGreaterOrEqual(const Key& key,
//     Node** prev) const {
//     Node* x = head_;
//     int level = GetMaxHeight() - 1;
//     while (true) {
//     Node* next = x->Next(level);
//     if (KeyIsAfterNode(key, next)) {
//     // Keep searching in this list
//     x = next;
//     } else {
//     if (prev != nullptr) prev[level] = x;
//     if (level == 0) {
//     return next;
//     } else {
//     // Switch to next list
//     level--;
//     }
//     }
//     }
// }
// }
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