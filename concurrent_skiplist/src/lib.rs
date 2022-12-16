

pub mod node;
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

use crate::node::Node;

const MAX_HEIGHT:i32 =12;
const K_BRANCHING:usize=4;
pub struct ConcurrentSkiplist<K:Ord,V>{
    // k:K,
    // v:V,
    max_height:AtomicI32,
    head:*mut Node<K, V>,
    insert_big_mu:Mutex<()>,
    mode:ConcurrentSkiplistMode,
    // herd:Herd,
    // free_list:Mutex<LinkedList<V>>,
}

unsafe impl<K:Ord,V> Send for ConcurrentSkiplist<K,V> {}
unsafe impl<K:Ord,V> Sync for ConcurrentSkiplist<K,V> {}

// impl<K:Ord,V> Clone for ConcurrentSkiplist<K,V> {
//     fn clone(&self) -> Self {
//         unreachable!();
//         // Self { max_height: Default::default(),  head: () }
//     }
// }
// fn check_prevs(prev:&[*mut Node<K, V>],len:i32){
//     for i in 0..len{
//         if prev[i].is_null(){
//             unreachable!()
//         }
//     }
// }
// impl<K:Ord,V> Copy for ConcurrentSkiplist<K,V> {}
#[derive(PartialEq)]
pub enum ConcurrentSkiplistMode{
    OneBigLock,
    NoLock
}
// unsafe impl<K:Ord,V> Send for ConcurrentSkiplist<K,V> {}
impl<K:Ord,V> ConcurrentSkiplist<K,V> {
    pub fn new_node_none(&self,height:i32) -> *mut Node<K, V> {
        // let mut vec=Vec::new();
        // let mut mu_v =Vec::new();
        // let vec:[AtomicPtr<Node<K,V>>;13] = ;
        // let ret=Node::alloc(Node{
        //     k: None,
        //     v: (None),
        //     next: Default::default(),
        //     // insert_mu: mu_v,
        // });
        // for i in 0..height+1 {
        //
        //     ret.next[i]=(AtomicPtr::new(std::ptr::null_mut()));
        //     // mu_v.push(Mutex::new(()));
        // };
        // ret
        // mu_v.resize(height as usize, Default::default());
        // std::boxed::into_raw();
        Box::into_raw(
            Box::new(
                Node{
                        k: None,
                        v: (None),
                        next: Default::default(),
                        // insert_mu: mu_v,
                    }

            )
        )
    }
    pub fn new_node(&self,k:K, v:V, height:i32) -> *mut Node<K, V> {
        // let mut vec=Vec::new();
        // // let mut mu_v =Vec::new();
        // for _ in 0..height+1 {
        //     vec.push(AtomicPtr::new(std::ptr::null_mut()));
        //     // mu_v.push(Mutex::new(()));
        // }
        // mu_v.resize(height as usize, Default::default());

        // self.herd.get().alloc(Node{
        //     k:Some(k),
        //     v: /*RwLock::new*/(Some(v)),
        //     next: Default::default(),
        //     // insert_mu: mu_v,
        // })
        Box::into_raw(
            Box::new(
                Node{
                    // k: None,
                    // v: (None),
                        k:Some(k),
                        v: /*RwLock::new*/(Some(v)),
                    next: Default::default(),
                    // insert_mu: mu_v,
                }

            )
        )
    }
    pub fn new(mode:ConcurrentSkiplistMode) -> ConcurrentSkiplist<K, V> {
        // let head=new_none(MAX_HEIGHT);

        let mut a=ConcurrentSkiplist{
            max_height: AtomicI32::new(1),
            head:null_mut(),
            // free_list: LinkedList::new(),
            insert_big_mu: Mutex::new(()),
            mode,
            // herd: Default::default(),
        };
        a.head=a.new_node_none(MAX_HEIGHT);
        a
    }
    fn get_max_height(&self) -> i32 {
        return self.max_height.load(Ordering::Relaxed);
    }
    fn random_height(&self)->i32{
        // let mut height:i32 = 1;
        // while height < MAX_HEIGHT && rand::thread_rng()
        //     .gen_range(0..K_BRANCHING)==0 {
        //     height+=1;
        // }
        // assert!(height > 0);
        // assert!(height <= MAX_HEIGHT);
        // return height;
        const MASK: u32 = 1 << (MAX_HEIGHT - 1);
        1 + (rand::random::<u32>() | MASK).trailing_zeros() as i32
    }
    unsafe fn key_is_after_node(&self, k:&K, node:*mut Node<K, V>) ->bool{
        return (node as u64!=0) && (
            (*node).unwrap_key_ref().cmp(k).is_lt()
            // compare_(n->key, key) < 0
        );
    }

    fn find_greater_or_equal(
        &self, k:&K,max_height:i32,
        mut prev: Option<&mut [*mut Node<K, V>]>) -> *mut Node<K, V> {
        let mut x=self.head;
        let mut level=max_height-1;

        loop {
            unsafe {
                let next=(*x).next(&self.mode,level);
                if self.key_is_after_node(k,next){
                    x=next;
                }else{
                    if let Some( prev)=prev.as_mut(){
                        prev[level as usize] = x;
                    }
                    if level == 0 {
                        // check_prevs(prev,self.get_max_height());
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
                (*x).next(self.mode.borrow(),level)
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
                (*x).next(self.mode.borrow(),level)
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
    fn insert(&self, key: K, value: V) -> Option<V> {
        // let _hold_big=if self.mode==ConcurrentSkiplistMode::OneBigLock{
        //     Some(self.insert_big_mu.lock())
        // }else{
        //     None
        // };

        //声明prev节点，代表插入位置的前一个节点
        let mut prev
            :[*mut Node<K, V>; MAX_HEIGHT as usize]
            = [std::ptr::null_mut();MAX_HEIGHT as usize];
        // 使用FindGreaterOrEqual函数找到第一个大于等于插入key的位置
        // 将对应节点的前驱全部存入prev

        let max_height=self.get_max_height();
// 使用随机数获取该节点的插入高度
        let height = self.random_height();
        // 大于当前skiplist 最高高度的话，将多出的来的高度的prev 设置为哨兵节点
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

            //更新最大高度
            self.max_height.store(height,Ordering::Relaxed);
            // max_height_.store(height, std::memory_order_relaxed);
        }
        // 创建要插入的节点对象
        let newnode=self.new_node(key,value,height);
        // println!("newnode {}",newnode as u64);
        //重试，必须保证最后一行被插入
        unsafe {
            let mut looptime =0;
            'retry: loop {
                let x = self.find_greater_or_equal(
                    (*newnode).unwrap_key_ref(),
                    max_height,Some(&mut prev));
                // println!("loop time {},x {}",looptime,x as u64);
                // Our data structure does not allow duplicate insertion
                // assert!(!x.is_null() && (key.cmp(x.k.unwrap()).is_ne());
                {
                    //已经存在对应key，把原本的值换出。
                    if !x.is_null() && ((*newnode).unwrap_key_ref().cmp((*x).unwrap_key_ref()).is_eq()) {
                        let mut v =None;

                        // unreachable!();
                        std::mem::swap(&mut (*newnode).v,&mut v);
                        // unreachable!("not support same key");
                        std::mem::swap(&mut (*x).v,&mut v);
                        // println!("{}",(*newnode).unwrap_key_ref());
                        return v;
                        // (*x).v
                    }
                }

                {
                // x = NewNode(key, height);
                // for (int i = 0; i < height; i++) {
                    let i=0;
                    let next=(*prev[i as usize]).nobarrier_next(
                        &self.mode,i,false
                    );
                    (*newnode).nobarrier_set_next(self.mode.borrow(),i, next);
                    if !next.is_null()
                        &&(*newnode).unwrap_key_ref().cmp((*next).unwrap_key_ref()).is_gt(){
                        //当前大于下一个，不对,重来
                        looptime+=1;
                        continue;
                    }
                    //prev下一个设置为x
                    if (*prev[i as usize]).cas_setnext(i,next,newnode){
                        //成功！！
                    }else{
                        looptime+=1;
                        continue;
                    }

                    for i in 1..height {
                        let next=(*prev[i as usize]).nobarrier_next(
                            &self.mode,i,false
                        );
                        (*newnode).nobarrier_set_next(self.mode.borrow(),i, next);
                        if !next.is_null()
                            &&(*newnode).unwrap_key_ref().cmp((*next).unwrap_key_ref()).is_gt(){
                            //当前大于下一个，不对,重来
                            // println!("break");
                            break 'retry;
                        }
                        //prev下一个设置为x
                        if
                        (*prev[i as usize]).cas_setnext(i,next,newnode){
                            //成功！！
                        }else{
                            // println!("break");
                            break 'retry;
                        }
                    }
                    break 'retry;
                }
            }
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
pub trait IndexOperate<K: Ord, V> {
    /// Get a range of keys in [key, range_end]
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    /// delete a range of keys in [key, range_end]
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}

impl<K:Ord,V> IndexOperate<K, V> for ConcurrentSkiplist<K,V>{
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>{
        let mut gr_or_eq =self.find_greater_or_equal(key, self.get_max_height(),None);

        let mut ret =Vec::<&V>::new();
        ret.reserve(1000);
        loop {
            unsafe {
                if gr_or_eq.is_null()||
                    (*gr_or_eq).unwrap_key_ref().cmp(range_end).is_ge(){
                    break;
                }
                if let Some(v)=(*gr_or_eq).v.as_ref(){
                    ret.push(v);
                }
                //切换到下一个
                gr_or_eq=(*gr_or_eq).next(self.mode.borrow(),0);
                //为null 或 >=end

            }
        }
        ret
    }
    /// delete a range of keys in [key, range_end]
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>{
        let mut gr_or_eq =self.find_greater_or_equal(key, self.get_max_height(),None);

        let mut ret =Vec::<V>::new();
        ret.reserve(1000);
        loop {
            unsafe {
                //为null 或 >=end
                if gr_or_eq.is_null()||
                    (*gr_or_eq).unwrap_key_ref().cmp(range_end).is_ge(){
                    break;
                }

                if (*gr_or_eq).v.is_some(){
                    let mut takeout:Option<V>=None;
                    std::mem::swap(&mut takeout, &mut (*gr_or_eq).v);
                    ret.push(takeout.unwrap());
                }
                //切换到下一个
                gr_or_eq=(*gr_or_eq).next(self.mode.borrow(),0);

            }
        }
        ret
    }
    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V>{
        self.insert(key,value)
    }

}