use std::sync::atomic::AtomicPtr;

pub(crate) struct Node<K,V>{
    k:K,
    v:V,
    next:Vec<AtomicPtr<Node<K,V>>>
}
impl <K,V> Node<K,V>{
    pub fn new(k:K, v:V, height:usize) -> Node<K, V> {
        Node{
            k,
            v,
            next: vec![AtomicPtr::new(std::ptr::null_mut());
                       height+1],
        }
    }
    pub fn next(&self){

    }
    pub fn set_next(&self){

    }
    pub fn nobarrier_next(&self){

    }
    pub fn nobarrier_set_next(&self){

    }

}