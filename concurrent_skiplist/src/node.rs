use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::RwLock;
// use atomic_option::AtomicOption;

pub struct Node<K,V>{
    pub k:Option<K>,
    pub v: Option<V>,
    next:Vec<AtomicPtr<Node<K,V>>>
}
impl <K,V> Node<K,V>{
    pub fn new_none(height:i32) -> *mut Node<K, V> {
        let mut vec=Vec::new();
        for _ in 0..height+1 {
            vec.push(AtomicPtr::new(std::ptr::null_mut()));
        }
        // std::boxed::into_raw();
        Box::into_raw(
            Box::new(
                Node{
                    k: None,
                    v: (None),
                    next: vec,
                }
            )
        )
    }
    pub fn new(k:K, v:V, height:i32) -> *mut Node<K, V> {
        let mut vec=Vec::new();
        for _ in 0..height+1 {
            vec.push(AtomicPtr::new(std::ptr::null_mut()));
        }
        Box::into_raw(
            Box::new(
                Node{
                    k:Some(k),
                    v: /*RwLock::new*/(Some(v)),
                    next: vec,
                }
            )
        )

    }
    pub fn unwrap_key_ref(&self)->&K{
        self.k.as_ref().unwrap()
    }
    pub fn next(&self, n:i32) -> *mut Node<K, V> {
        self.next[n as usize].load(Ordering::Acquire)
    }
    pub fn set_next(&self,n:i32,node:*mut Node<K, V>){
        self.next[n as usize].store(node,Ordering::Release);
    }
    pub fn nobarrier_next(&self,n:i32) -> *mut Node<K, V> {
        self.next[n as usize].load(Ordering::Relaxed)
    }
    pub fn nobarrier_set_next(&self,n:i32,node:*mut Node<K, V>){
        self.next[n as usize].store(node,Ordering::Relaxed);
    }

}