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

// use crate::AbstractOrd;

// pub use self::iter::*;

const MAX_HEIGHT: usize = 31;
type Ptr<T>     = Option<NonNull<T>>;
type Lanes<K,V>   = [AtomicPtr<Node<K,V>>; 1];// NB: Lanes is actually a variable sized array of lanes,
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
    inner: InnerNode<K,V>,
    lanes: Lanes<K,V>,
}

// NB: To allow optimizing repr of these fields
struct InnerNode<K,V> {
    elem: (K,V),
    height: u8,
}

impl<K:Ord,V> SkipListjjj<K,V> {
    pub fn new() -> SkipListjjj<K,V> {
        SkipListjjj {
            current_height: AtomicU8::new(8),
            lanes: Default::default(),
        }
    }

    pub fn insert(&self, k: K,v:V) -> Option<V> {
        insert::insert(&self.lanes[..], (k,v), &self.current_height)
    }
}

impl<K,V> SkipListjjj<K,V> {
    fn lanes(&self) -> &[AtomicPtr<Node<K,V>>] {
        let init = MAX_HEIGHT - self.current_height.load(Relaxed) as usize;
        &self.lanes[init..]
    }

    // pub fn get<'a, U: AbstractOrd<K,V> + ?Sized>(&'a self, elem: &U) -> Option<&T> {
    //     get::get(self.lanes(), elem)
    // }

    // pub fn elems(&self) -> Elems<'_, T> {
    //     Elems { nodes: self.nodes() }
    // }
    //
    // pub fn elems_mut(&mut self) -> ElemsMut<'_, T> {
    //     ElemsMut { nodes: self.nodes_mut() }
    // }
    //
    // pub fn into_elems(self) -> IntoElems<K,V> {
    //     let ptr = self.first();
    //     mem::forget(self);
    //     IntoElems { ptr }
    // }
    //
    // fn nodes(&self) -> Nodes<'_, T> {
    //     Nodes::new(self.first())
    // }
    //
    // fn nodes_mut(&mut self) -> NodesMut<'_, T> {
    //     NodesMut::new(self.first())
    // }
    //
    // fn first(&self) -> Ptr<Node<K,V>> {
    //     NonNull::new(self.lanes[MAX_HEIGHT - 1].load(Acquire))
    // }
}

impl<K,V> Node<K,V> {
    fn alloc(elem: (K,V), max_height: &AtomicU8) -> NonNull<Node<K,V>> {
        let height = random_height();
        max_height.fetch_max(height as u8, Relaxed);
        unsafe {
            let layout = Node::<K,V>::layout(height);
            let ptr = alloc::alloc_zeroed(layout) as *mut Node<K,V>;
            (*ptr).inner.height = height as u8;
            ptr::write(&mut (*ptr).inner.elem as *mut (K,V), elem);
            NonNull::new_unchecked(ptr)
        }
    }

    unsafe fn dealloc(&mut self) -> (K,V) {
        let layout = Node::<K,V>::layout(self.height());
        let elem = ptr::read(&mut self.inner.elem);
        alloc::dealloc(self as *mut Node<K,V> as *mut u8, layout);
        elem
    }

    fn next(&self) -> Ptr<Node<K,V>> {
        NonNull::new(self.lanes().last().unwrap().load(Acquire))
    }

    fn lanes(&self) -> &[AtomicPtr<Node<K,V>>] {
        #[repr(C)]
        struct LanesPtr<K,V> {
            lanes: *const Lanes<K,V>,
            height: usize,
        }

        let lanes = &self.lanes as *const Lanes<K,V>;
        let height = self.height();
        unsafe { mem::transmute(LanesPtr { lanes, height }) }
    }

    fn height(&self) -> usize {
        self.inner.height as usize
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
//         iter.into_iter().for_each(|elem| {
//             self.insert(elem);
//         });
//     }
// }
//
// impl<'a, T: AbstractOrd<K,V> + Copy> Extend<&'a T> for SkipListjjj<K,V> {
//     fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
//         iter.into_iter().for_each(|&elem| {
//             self.insert(elem);
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