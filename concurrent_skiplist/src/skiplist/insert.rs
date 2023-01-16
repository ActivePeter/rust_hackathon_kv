use std::cmp::Ordering::*;
use std::mem::ManuallyDrop;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicPtr, AtomicU8};
use std::sync::atomic::Ordering::{Acquire, AcqRel, Release};
use std::sync::mpsc;
use crate::IndexOperate;
use crate::skiplist::SkipListjjj;

use super::{Ptr, Node, MAX_HEIGHT};
impl<K:Ord,V> SkipListjjj<K,V> {
    pub(crate) fn insert(&self, k: K, v:V) -> Option<V> {
        let max_height=&self.current_height;
        //总体思路是，确保最下层的被全部链接，tower的build则随缘，失败就跳出
        let mut kv: ManuallyDrop<(K,V)> = ManuallyDrop::new((k,v));
        // let mut kv_ptr: NonNull<(K,V)> = NonNull::from(&*kv);
        let mut new_node: Ptr<Node<K,V>> = None;
        //重试，直到成功插入最后一行
        'retry: loop {
            let mut lanes = &self.lanes[..];
            let mut height = lanes.len();
            let mut prevs_succs: [(*const AtomicPtr<Node<K,V>>, *mut Node<K,V>); MAX_HEIGHT];
            prevs_succs = [(ptr::null(), ptr::null_mut()); MAX_HEIGHT];
            //搜索
            'across: while height > 0 {
                'down: for atomic_ptr in lanes {
                    let ptr: Ptr<Node<K,V>> = NonNull::new(atomic_ptr.load(Acquire));
                    match ptr {
                        //到达行尾，向下
                        None        => {
                            height -= 1;
                            prevs_succs[height] = (atomic_ptr, ptr::null_mut());
                            continue 'down;
                        }

                        //还未到行尾，判断当前节点与key的大小
                        Some(ptr)   => unsafe {
                            let node = &mut*ptr.as_ptr();
                            let (kref,vref) = &*kv;//kv_ptr.as_ref();

                            match kref.cmp(&node.k.as_ref().unwrap()) {
                                //相等，换出我们要的结果
                                Equal   => match &mut new_node {
                                    Some(new_node)  => {
                                        // let mut aa =None;
                                        // std::mem::swap(&mut aa,&mut new_node.as_mut().kv);
                                        // =None;
                                        let mut aa =new_node.as_mut().dealloc().unwrap();
                                        std::mem::swap(&mut aa.1, node.v.as_mut().unwrap());
                                        return Some(aa.1);
                                    }
                                    None            => {
                                        let mut aa =ManuallyDrop::take(&mut kv);
                                        if(node.v.as_mut().is_some()){
                                            std::mem::swap(&mut aa.1, node.v.as_mut().unwrap());
                                            return Some(aa.1);
                                        }else{
                                            return None;
                                        }

                                    }
                                }

                                // 当前节点小于key，
                                Less    => {
                                    height -= 1;
                                    prevs_succs[height] = (atomic_ptr, ptr.as_ptr());
                                    continue 'down;
                                }

                                // 当前节点大于key，则key找到了这一行的插入位置
                                // 切换到下一层的行
                                Greater => {
                                    lanes = &node.lanes()[(node.height() - height)..];
                                    continue 'across;
                                }
                            }
                        }
                    }
                }
            }
            //到达最底层循环就结束了
            // 创建新节点
            let new_node: NonNull<Node<K,V>> = match new_node {
                // 不是第一次循环，以及retry过了，之前以及创建过了新节点
                Some(new_node)  => new_node,

                // 创建新节点
                None        => {
                    let kv = unsafe { ManuallyDrop::take(&mut kv) };
                    let node = Node::alloc(kv, max_height);
                    // kv_ptr = unsafe { NonNull::from(&node.as_ref().inner.kv) };
                    new_node = Some(node);
                    new_node.unwrap()
                }
            };


            let new_node_addr = new_node.as_ptr();
            let new_node_lanes = unsafe { new_node.as_ref().lanes() };
            let mut inserted = false;

            // 新节点从下网上创建行
            'insert: for (new, &(pred, succ)) in new_node_lanes.iter().rev().zip(&prevs_succs) {
                let pred: & AtomicPtr<Node<K,V>> = unsafe { &*pred };

                new.store(succ, Release);

                match pred.compare_exchange(succ, new_node_addr, Acquire,Acquire).is_ok() {
                    // 插入成功，设置标志位，继续
                    true                => inserted = true,

                    // 插入失败 最底层没有成功插入
                    false if !inserted  => continue 'retry,

                    // 插入失败 最底层已经成功插入
                    false               => break 'insert,
                }
            }

            return None;
        }
    }
}
