use std::sync::Arc;
use std::thread;
use concurrent_skiplist::{ConcurrentSkiplist, IndexOperate};
use rand::Rng;
use std::collections::BTreeMap;

//todo
// 单线程批量插入，验证结果
// 单线程批量删除，验证返回值以及是否删除成功
// 多个线程同时插入（每个线程的key不冲突），验证结果
// 多个线程同时删除（每个线程key不冲突），验证结果
// 待补充

#[test]
fn insert() {
    let mut rng = rand::thread_rng();
    let mut std_map = BTreeMap::<i32,i32>::new();
    let our_map = ConcurrentSkiplist::<i32, i32>::new();

    for key in 1..10000 {
        let value = rng.gen_range(-10000..=10000);
        std_map.insert(key, value);
        our_map.insert_or_update(key, value);
    }
    
    for key in 1..10000 {
        // FIXME 怎么才能 get value
        assert_eq!(std_map.get(&key), our_map.get(&key));
    }
}


#[test]
fn multithread(){
    let map=
        Arc::new(ConcurrentSkiplist::<i32, i32>::new());
    for i in 1..10 {
        let map_=map.clone();
        thread::spawn(move || {
            map_.insert(i,i);
        });
    }
}