use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Mutex, RwLock};
use crate::ConcurrentSkiplistMode;
use crate::ConcurrentSkiplistMode::NoLock;
// use atomic_option::AtomicOption;

pub struct Node<K,V>{
    pub k:Option<K>,
    pub v: Option<V>,
    pub(crate) next:[AtomicPtr<Node<K,V>>;13],
    // pub insert_mu:Vec<Mutex<()>>
}
impl <K,V> Node<K,V>{

    pub fn unwrap_key_ref(&self)->&K{
        self.k.as_ref().unwrap()
    }
    pub fn next(&self,mode:&ConcurrentSkiplistMode, n:i32) -> *mut Node<K, V> {
        // if *mode== NoLock {

            // let _hold1=self.insert_mu[n as usize].lock();
            // self.next[n as usize].load(Ordering::Acquire)
        // }else{

            self.next[n as usize].load(Ordering::Acquire)
        // }
    }
    pub fn cas_setnext(&self, n:i32, exp:*mut Node<K, V>, node:*mut Node<K, V>) -> bool {
        self.next[n as usize].compare_exchange(
            exp,node,Ordering::Acquire,Ordering::Acquire
        ).is_ok()
    }
    pub fn set_next(&self,mode:&ConcurrentSkiplistMode,n:i32,node:*mut Node<K, V>,locked:bool){
        // if *mode== NoLock &&!locked{
        //     // let _hold1=self.insert_mu[n as usize].lock();
        //     self.next[n as usize].store(node,Ordering::Release);
        // }else
        {
            self.next[n as usize].store(node,Ordering::Release);
        }
    }
    pub fn nobarrier_next(&self,mode:&ConcurrentSkiplistMode,n:i32,locked:bool) -> *mut Node<K, V> {
        // if locked ||*mode!= NoLock {
        //     self.next[n as usize].load(Ordering::Relaxed)
        // }else
        {
            // let _hold1=self.insert_mu[n as usize].lock();
            self.next[n as usize].load(Ordering::Relaxed)
        }

    }
    pub fn nobarrier_set_next(&self,mode:&ConcurrentSkiplistMode,n:i32,node:*mut Node<K, V>){
        // if *mode== NoLock {
        //     // let _hold1=self.insert_mu[n as usize].lock();
        //     self.next[n as usize].store(node,Ordering::Relaxed);
        // }else
        {
            self.next[n as usize].store(node,Ordering::Relaxed);
        }
    }

}