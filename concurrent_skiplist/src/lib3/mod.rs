// mod get;
mod insert;
// mod iter;

use std::alloc;
use std::cmp;
use std::fmt;
use std::iter::FromIterator;
use std::mem;
use std::ptr::{self, NonNull};
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
    kv: Option<(K,V)>,
    height: u8,
    lanes: [AtomicPtr<Node<K,V>>; MAX_HEIGHT+1],
}


impl<K:Ord,V> SkipListjjj<K,V> {
    pub fn new() -> SkipListjjj<K,V> {
        SkipListjjj {
            current_height: AtomicU8::new(8),
            lanes: Default::default(),
            // herd: Default::default(),
        }
    }
    // fn alloc_node(&self,kv: (K,V), max_height: &AtomicU8) -> NonNull<Node<K,V>> {
    //     let height = random_height();
    //     max_height.fetch_max(height as u8, Relaxed);
    //     unsafe {
    //         NonNull::new_unchecked(self.herd.get().alloc(
    //                                Node{
    //                                    kv: Some(kv),
    //                                    height: height as u8,
    //                                    lanes: Default::default(),
    //                                })
    //
    //         )
    //     }
    //     // unsafe {
    //     //     let layout = Node::<K,V>::layout(height);
    //     //     let ptr = alloc::alloc_zeroed(layout) as *mut Node<K,V>;
    //     //     (*ptr).height = height as u8;
    //     //     ptr::write(&mut (*ptr).kv as *mut (K,V), kv);
    //     //     NonNull::new_unchecked(ptr)
    //     // }
    // }
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
        unsafe {
            NonNull::new_unchecked(Box::into_raw(
                Box::new(
                    Node{
                        kv: Some(kv),
                        height: height as u8,
                        lanes: Default::default(),
                    }
                )
            )
            )
        }
        // unsafe {
        //     let layout = Node::<K,V>::layout(height);
        //     let ptr = alloc::alloc_zeroed(layout) as *mut Node<K,V>;
        //     (*ptr).height = height as u8;
        //     ptr::write(&mut (*ptr).kv as *mut (K,V), kv);
        //     NonNull::new_unchecked(ptr)
        // }
    }

    unsafe fn dealloc(&mut self) -> (K,V) {
        let mut r =None;
        std::mem::swap(&mut r,&mut self.kv);
        let n=Box::from_raw(self);
        r.unwrap()
    }

    fn next(&self) -> Ptr<Node<K,V>> {
        NonNull::new(self.lanes().last().unwrap().load(Acquire))
    }

    fn lanes(&self) -> &[AtomicPtr<Node<K,V>>] {
        // #[repr(C)]
        // struct LanesPtr<K,V> {
        //     lanes: *const Lanes<K,V>,
        //     height: usize,
        // }
        //
        // let lanes = &self.lanes as *const Lanes<K,V>;
        // let height = self.height();
        // unsafe { mem::transmute(LanesPtr { lanes, height }) }
        &self.lanes[0..self.height()]
    }

    fn height(&self) -> usize {
        self.height as usize
    }

    fn layout(height: usize) -> alloc::Layout {
        let size = ((height + 1) * mem::size_of::<usize>()) + mem::size_of::<(K,V)>();
        let align = cmp::max(mem::align_of::<(K,V)>(), mem::align_of::<usize>());
        unsafe {
            alloc::Layout::from_size_align_unchecked(size, align)
        }
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

// impl<T: AbstractOrd<K,V>> Extend<K,V> for SkipListjjj<K,V> {
//     fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         iter.into_iter().for_each(|kv| {
//             self.insert(kv);
//         });
//     }
// }
//
// impl<'a, T: AbstractOrd<K,V> + Copy> Extend<&'a T> for SkipListjjj<K,V> {
//     fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
//         iter.into_iter().for_each(|&kv| {
//             self.insert(kv);
//         });
//     }
// }
//
// impl<T: AbstractOrd<K,V>> FromIterator<K,V> for SkipListjjj<K,V> {
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
//         let mut list = Self::new();
//         list.extend(iter);
//         list
//     }
// }

fn random_height() -> usize {
    const MASK: u32 = 1 << (MAX_HEIGHT - 1);
    1 + (rand::random::<u32>() | MASK).trailing_zeros() as usize
}