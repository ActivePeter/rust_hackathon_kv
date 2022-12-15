

pub mod node;
use crate::node::Node;
const MAX_HEIGHT:i32 =12;
const K_BRANCHING:usize=4;
pub struct ConcurrentSkiplist<K:Ord,V>{
    k:K,
    v:V,
    // max_height,
    head:*mut Node<K, V>
}

impl<K:Ord,V> ConcurrentSkiplist<K,V> {
    pub fn new(){

    }
    fn random_height(&self)->i32{
        let mut height:i32 = 1;
        // while height < MAX_HEIGHT && rnd_.OneIn(K_BRANCHING) {
        //     height+=1;
        // }
        assert!(height > 0);
        assert!(height <= MAX_HEIGHT);
        return height;

    }
    unsafe fn key_is_after_node(&self, k:&K, node:*mut Node<K, V>) ->bool{
        return (!node.is_null()) && (
            (*node).k.cmp(k).is_lt()
            // compare_(n->key, key) < 0
        );
    }
    unsafe fn find_greater_or_equal(&self, k:&K) -> *mut Node<K, V> {
        let mut x=self.head;
        let mut level=MAX_HEIGHT-1;
        loop {
            let next=(*x).next(level);
            if self.key_is_after_node(k,next){
                x=next;
            }else{
                // TODO prev
                // if (prev != nullptr) prev[level] = x;
                if level == 0 {
                    return next;
                } else {
                    // Switch to next list
                    level-=1;
                }
            }
        }
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