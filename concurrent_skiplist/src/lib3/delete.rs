use std::cmp::Ordering;
use std::ptr::{NonNull, null_mut};
use std::sync::atomic::Ordering::Acquire;
use crate::IndexOperate;
use crate::lib3::{Node, SkipListjjj};

impl<K:Ord,V>  SkipListjjj<K,V>{
    /// delete a range of keys in [key, range_end]
    pub(crate) fn inner_delete(&self, key: &K, range_end: &K) -> Vec<V>{
        let mut lanes=&self.lanes[..];
        let mut height = lanes.len();
        let mut v =vec![];

        let mut ptr = NonNull::new(null_mut());
        'across: while height > 0 {
            'down: for atomic_ptr in lanes {
                ptr=NonNull::new(atomic_ptr.load(Acquire));
                match ptr {
                    None        => {
                        height -= 1;
                        continue 'down;
                    }
                    Some(ptr)  => {
                        let node: & Node<K,V> = unsafe { &*ptr.as_ptr() };

                        match key.cmp(&node.k.as_ref().unwrap()) {
                            // Equal   => return Some(&node.kv.),

                            Ordering::Less    => {
                                height -= 1;
                                continue 'down;
                            }

                            Ordering::Equal=>{
                                //找到了
                                break 'across;
                            }
                            Ordering::Greater => {//第一个greater
                                //切换lane
                                lanes = &node.lanes()[(node.height() - height)..];
                                continue 'across;
                            }

                        }
                    }
                }
            }
        }
        unsafe {
            while ptr.is_some() &&ptr.as_ref().unwrap().as_ref().k.as_ref().unwrap().cmp(range_end).is_lt() {
                // assert!(ptr.as_ref().unwrap().as_ref().k.as_ref().unwrap().cmp(
                //     key
                // ).is_ge());
                let mut into =None;
                std::mem::swap(&mut into,&mut ptr.as_mut().unwrap().as_mut().v);
                if into.is_some(){

                    v.push(into.unwrap());
                }
                ptr= ptr.as_ref().unwrap().as_ref().next();
            }
        }
        v
    }

}