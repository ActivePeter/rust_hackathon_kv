

// pub mod lib2;
pub mod lib3;

use std::borrow::{Borrow, BorrowMut};
use std::collections::LinkedList;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI32, AtomicPtr, Ordering};
use bumpalo_herd::Herd;
use parking_lot::Mutex;
// use std::sync::Mutex;
use rand::Rng;


const MAX_HEIGHT:i32 =12;

/// Operations of Index
/// trait with generic type
pub trait IndexOperate<K: Ord, V> {
    /// Get a range of keys in [key, range_end]
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    /// delete a range of keys in [key, range_end]
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}
