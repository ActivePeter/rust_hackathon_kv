

pub mod node;

use std::sync::atomic::{AtomicI32, Ordering};
use crate::node::Node;
const MAX_HEIGHT:i32 =12;
const K_BRANCHING:usize=4;
pub struct ConcurrentSkiplist<K:Ord,V>{
    // k:K,
    // v:V,
    max_height:AtomicI32,
    head:*mut Node<K, V>
}

impl<K:Ord,V> ConcurrentSkiplist<K,V> {
    pub fn new() -> ConcurrentSkiplist<K, V> {
        let head=Node::new_none(MAX_HEIGHT);

        ConcurrentSkiplist{
            max_height: AtomicI32::new(0),
            head,
        }
    }
    fn get_max_height(&self) -> i32 {
        return self.max_height.load(Ordering::Relaxed);
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
            (*node).unwrap_key_ref().cmp(k).is_lt()
            // compare_(n->key, key) < 0
        );
    }
    fn find_greater_or_equal(
        &self, k:&K,
        mut prev: Option<&mut [*mut Node<K, V>]>) -> *mut Node<K, V> {
        let mut x=self.head;
        let mut level=self.get_max_height()-1;
        loop {
            unsafe {
                let next=(*x).next(level);
                if self.key_is_after_node(k,next){
                    x=next;
                }else{
                    if let Some( prev)=prev.as_mut(){
                        prev[level as usize] = x;
                    }
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
    fn find_less_than(&self, k:&K) -> *mut Node<K, V> {
        let mut x = self.head;
        let mut level = self.get_max_height() - 1;
        loop {
            // assert(x == head_ || compare_(x->key, key) < 0);
            let next = unsafe {
                (*x).next(level)
            };
            if next.is_null() ||
                unsafe {
                    (*next).unwrap_key_ref().cmp(k).is_ge()
                }
                // compare_(next->key, key) >= 0
            {
                if level == 0 {
                    return x;
                } else {
                    // Switch to next list
                    level -=1;
                }
            } else {
                x = next;
            }
        }
    }
    fn find_last(&self) -> *mut Node<K, V> {
        let mut x = self.head;
        let mut level = self.get_max_height() - 1;
        loop {
            let mut next =unsafe{
                (*x).next(level)
            };
            if next.is_null() {
                if level == 0 {
                    return x;
                } else {
                // Switch to next list
                    level-=1;
                }
            } else {
                x = next;
            }
        }
    }
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        // TODO(opt): We can use a barrier-free variant of FindGreaterOrEqual()
        // here since Insert() is externally synchronized.
        let mut prev
            :[*mut Node<K, V>; MAX_HEIGHT as usize]
            = [std::ptr::null_mut();MAX_HEIGHT as usize];
        let mut x = self.find_greater_or_equal(
            &key, Some(&mut prev));

        // Our data structure does not allow duplicate insertion
        // assert!(x == nullptr || (key.cmp(x.k.unwrap()).is_ne());


        let height = self.random_height();
        let max_height=self.get_max_height();
        if height > max_height {
            for i in max_height..height{
                // for (int i = GetMaxHeight(); i < height; i++)

                prev[i as usize] = self.head;
            }
            // It is ok to mutate max_height_ without any synchronization
            // with concurrent readers.  A concurrent reader that observes
            // the new value of max_height_ will see either the old value of
            // new level pointers from head_ (nullptr), or a new value set in
            // the loop below.  In the former case the reader will
            // immediately drop to the next level since nullptr sorts after all
            // keys.  In the latter case the reader will use the new node.
            self.max_height.store(height,Ordering::Relaxed);
            // max_height_.store(height, std::memory_order_relaxed);
        }

        x=Node::new(key,value,height);
        // x = NewNode(key, height);
        // for (int i = 0; i < height; i++) {
        for i in 0..height {
            // NoBarrier_SetNext() suffices since we will add a barrier when
            // we publish a pointer to "x" in prev[i].
            // let v=prev[i].;
            unsafe {
                (*x).nobarrier_set_next(
                    i,
                    (*prev[i as usize]).nobarrier_next(i));
                (*prev[i as usize]).set_next(i,x);
            }

            // x->NoBarrier_SetNext(i, prev[i]->NoBarrier_Next(i));
            // prev[i]->SetNext(i, x);
        }
        None
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
        self.insert(key,value)
    }

}