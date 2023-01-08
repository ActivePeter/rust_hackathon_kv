use std::cmp::Ordering;
use std::ptr::{NonNull, null_mut};
use std::sync::atomic::Ordering::Acquire;
use crate::IndexOperate;
use crate::skiplist::{Node, Ptr, SkipListjjj};

impl<K:Ord,V> SkipListjjj<K,V>{
    pub fn inner_get(&self, key: &K, range_end: &K) -> Vec<&V>{
        let mut lanes=&self.lanes[..];
        let mut height = lanes.len();
        let mut v =vec![];
        let mut ptr = NonNull::new(null_mut());
        let (sender,rx)=std::sync::mpsc::channel::<i32>();

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
                if ptr.as_ref().unwrap().as_ref().v.as_ref().is_some(){
                    v.push(ptr.as_ref().unwrap().as_ref().v.as_ref().unwrap());
                }
                ptr= ptr.as_ref().unwrap().as_ref().next();
            }
        }

        return v;
    }
}