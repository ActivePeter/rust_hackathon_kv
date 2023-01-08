// mod get;
mod insert;
mod get;
mod trait_impl;
mod delete;
// mod iter;

use std::alloc;
use std::cmp;
use std::fmt;
use std::iter::FromIterator;
use std::mem;
use std::ptr::{self, NonNull, null_mut};
use std::sync::atomic::{AtomicPtr, AtomicU8};
use std::sync::atomic::Ordering::{Relaxed, Acquire};
use bumpalo_herd::Herd;

// use crate::AbstractOrd;

// pub use self::iter::*;

const MAX_HEIGHT: usize = 31;
type Ptr<T>     = Option<NonNull<T>>;
// containing at least one lane, but possibly as many as
// MAX_HEIGHT.

pub struct SkipListjjj<K,V> {
    current_height: AtomicU8,
    lanes: [AtomicPtr<Node<K,V>>; MAX_HEIGHT],
}

unsafe impl<K,V> Send for SkipListjjj<K,V> { }
unsafe impl<K,V> Sync for SkipListjjj<K,V> { }

#[repr(C)] // NB: repr(C) necessary to avoid reordering lanes field, which must be the tail
struct Node<K,V> {
    k: Option<K>,
    v: Option<V>,
    height: u8,
    lanes: Vec<AtomicPtr<Node<K,V>>>// MAX_HEIGHT+1],
}


impl<K:Ord,V> SkipListjjj<K,V> {
    pub fn new() -> SkipListjjj<K,V> {
        SkipListjjj {
            current_height: AtomicU8::new(8),
            lanes: Default::default(),
            // herd: Default::default(),
        }
    }
}

impl<K,V> SkipListjjj<K,V> {
    fn lanes(&self) -> &[AtomicPtr<Node<K,V>>] {
        let init = MAX_HEIGHT - self.current_height.load(Relaxed) as usize;
        &self.lanes[init..]
    }
}

impl<K,V> Node<K,V> {
    fn alloc(kv: (K,V), max_height: &AtomicU8) -> NonNull<Node<K,V>> {
        let height = random_height();
        max_height.fetch_max(height as u8, Relaxed);
        let mut lanes =Vec::with_capacity(height);
        for _ in 0..height{
            lanes.push(AtomicPtr::default());
        }
        unsafe {
            NonNull::new_unchecked(Box::into_raw(
                Box::new(
                    Node{
                        k: Some(kv.0),
                        v: Some(kv.1),
                        height: height as u8,
                        lanes,
                        // v: None,
                    }
                )
            )
            )
        }
    }

    unsafe fn dealloc(&mut self) -> Option<(K,V)> {
        let mut k =None;
        let mut v=None;
        std::mem::swap(&mut k,&mut self.k);
        std::mem::swap(&mut v,&mut self.v);
        let n=Box::from_raw(self);
        if v.is_some(){
            Some((k.unwrap(),v.unwrap()))
        }else{
            None
        }
    }

    fn next(&self) -> Ptr<Node<K,V>> {
        NonNull::new(self.lanes().last().unwrap().load(Acquire))
    }

    fn lanes(&self) -> &[AtomicPtr<Node<K,V>>] {
        &self.lanes[0..self.height()]
    }

    fn height(&self) -> usize {
        self.height as usize
    }

}


impl<K,V> Drop for SkipListjjj<K,V> {
    fn drop(&mut self) {
        unsafe {
            let mut first=self.lanes[MAX_HEIGHT - 1].load(Acquire);
            while !first.is_null() {
                let bak=(*first).next();
                (*first).dealloc();
                if bak.is_some(){
                    first=bak.unwrap().as_ptr();
                }else{
                    break;
                }
            }
        }
    }
}

fn random_height() -> usize {
    const MASK: u32 = 1 << (MAX_HEIGHT - 1);
    1 + (rand::random::<u32>() | MASK).trailing_zeros() as usize
}