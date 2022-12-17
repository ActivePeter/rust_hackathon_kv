use std::cmp::Ordering::*;
use std::mem::ManuallyDrop;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicPtr, AtomicU8};
use std::sync::atomic::Ordering::{Acquire, AcqRel, Release};
use crate::IndexOperate;
use crate::lib3::SkipListjjj;

// use crate::AbstractOrd;
use super::{Ptr, Node, MAX_HEIGHT};
impl<K:Ord,V> SkipListjjj<K,V> {
    pub(crate) fn insert(&self, k: K, v:V) -> Option<V> {
        let max_height=&self.current_height;
        // insert::insert(&self.lanes[..], (k,v), &self.current_height)

        // This wonky memory set up is necessary to handle retry iteration: we do
        // not know we need to retry the insertion until after we have already
        // allocated a node for this kvent. We are faced with a dilemma because
        // of this retry issue:
        //
        //  - The first time searching, we do not know if we need to insert. To
        //    avoid unnecessary allocations, we do not allocate a node for this
        //    kvent until the first search has turned up empty.
        //  - When we discover we need to retry, we have already allocated the
        //    node for the kvent, moving the kvent into the heap. We do not
        //    want to deallocate it for the search, because we would likely need
        //    to allocate it again once we find that the kvent is still not
        //    present.
        //
        // For this reason, we access the kvent during search through the
        // "kv_ptr" variable, which could point either to the kvent in the
        // stack or the kvent in the heap. We manage dropping/forgetting the kv
        // correctly using a ManuallyDrop wrapper. The `new_node` pointer is used
        // to track if the node has been allocated (it has if it is non-null).
        let mut kv: ManuallyDrop<(K,V)> = ManuallyDrop::new((k,v));
        // let mut kv_ptr: NonNull<(K,V)> = NonNull::from(&*kv);
        let mut new_node: Ptr<Node<K,V>> = None;

        // The 'retry loop handles retrying an insert when it fails completely
        // (that is, when there  is contention inserting this node into the lowest
        // lane which contains all nodes). During the insert loop, there is a
        // single `continue 'retry;`; except for that, the 'retry loop should be
        // exited on the first iteration.
        'retry: loop {
            let mut lanes = &self.lanes[..];
            let mut height = lanes.len();

            // The immediate predecessor and successor of this kvent in each
            // lane of the skiplist. The predecessor pointer is a pointer to the
            // actual AtomicPtr in that lane of that node, which will be set to
            // point to this kvent. The successor pointer is just the address
            // of the successor node, which this node's pointer will be set to,
            // and which will be used in a compare and swap operation on the
            // predecessor pointer.
            let mut spots : [(*const AtomicPtr<Node<K,V>>, *mut Node<K,V>); MAX_HEIGHT];
            spots = [(ptr::null(), ptr::null_mut()); MAX_HEIGHT];

            // This is very similar to the search in get, but we track the
            // predecessors and successors in each lane usin the `spots` variable.
            // We iterate across the list, visting different nodes, and down each
            // node's list of lanes, until we find the point in the lowest lane at
            // which we are to insert our new node.
            'across: while height > 0 {
                'down: for atomic_ptr in lanes {
                    let ptr: Ptr<Node<K,V>> = NonNull::new(atomic_ptr.load(Acquire));

                    match ptr {
                        // If the pointer is null, we are at the end of this lane
                        // and we should move downward.
                        None        => {
                            height -= 1;
                            spots[height] = (atomic_ptr, ptr::null_mut());
                            continue 'down;
                        }

                        // If not, we will do a comparison between the kvent
                        // to be inserted and the kvent at this node.
                        Some(ptr)   => unsafe {
                            let node = &mut*ptr.as_ptr();
                            let (kref,vref) = &*kv;//kv_ptr.as_ref();

                            match kref.cmp(&node.k.as_ref().unwrap()) {
                                // If they are equal, this kvent has already
                                // been inserted into the list, and we need to
                                // return the kvent we attempted to insert. The
                                // logic for this depends on whether or not we've
                                // already allocated a node (in a previous
                                // iteration of the 'retry loop). If we have, we
                                // must deallocate that node to avoid leaking it.
                                Equal   => match &mut new_node {
                                    Some(new_node)  => {
                                        // let mut aa =None;
                                        // std::mem::swap(&mut aa,&mut new_node.as_mut().kv);
                                        // =None;
                                        let mut aa =new_node.as_mut().dealloc();
                                        std::mem::swap(&mut aa.1, node.v.as_mut().unwrap());
                                        return Some(aa.1);
                                    }
                                    None            => {
                                        let mut aa =ManuallyDrop::take(&mut kv);
                                        std::mem::swap(&mut aa.1, node.v.as_mut().unwrap());
                                        return Some(aa.1);
                                    }
                                }

                                // If the kvent to be inserted is less than the
                                // kvent in this node, we want to move down the
                                // lanes.
                                Less    => {
                                    height -= 1;
                                    spots[height] = (atomic_ptr, ptr.as_ptr());
                                    continue 'down;
                                }

                                // If the kvent to be inserted is greater than
                                // the kvent in this node, we want to move across
                                // the list, iterating through the lanes in that
                                // node.
                                Greater => {
                                    lanes = &node.lanes()[(node.height() - height)..];
                                    continue 'across;
                                }
                            }
                        }
                    }
                }
            }

            // Allocate the new node if it hasn't already been allocated.
            let new_node: NonNull<Node<K,V>> = match new_node {
                // If the node is not null, its already been allocated and there is
                // no work to be done.
                Some(new_node)  => new_node,

                // Otherwise, allocate the node, move the kvent into it, and
                // reset the kv_ptr to point into the heap instead of to the old
                // location on the stack.
                None        => {
                    let kv = unsafe { ManuallyDrop::take(&mut kv) };
                    let node = Node::alloc(kv, max_height);
                    // kv_ptr = unsafe { NonNull::from(&node.as_ref().inner.kv) };
                    new_node = Some(node);
                    new_node.unwrap()
                }
            };

            // The insert loop iterates upward from the lowest lane of this node
            // to its highest, attempting to insert it at each point, performing
            // an atomic compare and swap to identify conflicts with concurrent
            // insertions. Because the node *must* be inserted into at least one
            // lane, if the lowest lane fails we do a complete retry, but if any
            // higher lanes fail, we simply consider the insertion successful,
            // leaving the list slighter flatter than it should be.
            let new_node_addr = new_node.as_ptr();
            let new_node_lanes = unsafe { new_node.as_ref().lanes() };
            let mut inserted = false;

            'insert: for (new, &(pred, succ)) in new_node_lanes.iter().rev().zip(&spots) {
                let pred: & AtomicPtr<Node<K,V>> = unsafe { &*pred };

                new.store(succ, Release);

                match pred.compare_exchange(succ, new_node_addr, Acquire,Acquire).is_ok() {
                    // We successfully inserted the node into at least one lane,
                    // we note that for future iterations.
                    true                => inserted = true,

                    // Because the node has not been inserted yet, we need to retry
                    // the entire insertion on this failure.
                    false if !inserted  => continue 'retry,

                    // Because the node has been inserted into at least one lane
                    // of the list, we just finish the insertion here.
                    false               => break 'insert,
                }
            }

            return None;
        }
    }
}


// pub(super) fn insert<K,V>(lanes: &[AtomicPtr<Node<K,V>>], kv: (K,V), max_height: &AtomicU8)
//                             -> Option<V>
//     where K: Ord
// {
//
// }