use concurrent_skiplist::{ConcurrentSkiplist, ConcurrentSkiplistMode, IndexOperate};
use rand::Rng;
use std::{collections::BTreeMap, sync::Arc, thread};

#[test]
/// 单线程批量插入，验证 ConcurrentSkiplist 内容
fn insert() {
    // 初始化
    let mut rng = rand::thread_rng();
    let mut std_map = BTreeMap::<i32, i32>::new();
    let our_map = ConcurrentSkiplist::<i32, i32>::new(
        ConcurrentSkiplistMode::NoLock);

    // 批量插入数据
    for key in 1..=10000 {
        let value = rng.gen_range(-10000..=10000);
        std_map.insert(key, value);
        our_map.insert_or_update(key, value);
    }

    // 验证 ConcurrentSkiplist 内容
    for key in 1..=10000 {
        let wanted = std_map.get(&key);

        let will_be_none = our_map.get(&key, &key);
        assert_eq!(will_be_none.len(), 0);

        let will_be_single = our_map.get(&key, &(key + 1));
        assert_eq!(will_be_single.len(), 1);
        assert_eq!(will_be_single[0], wanted.unwrap());
    }

    // TODO 检查 insert_or_update 已存在 key 的情况
    // TODO 使用不存在的 key，our_map.get 将会返回什么？
}

#[test]
/// 单线程批量删除，验证 ConcurrentSkiplist 内容
fn delete() {
    // 初始化
    let mut rng = rand::thread_rng();
    let mut std_map = BTreeMap::<i32, i32>::new();
    let our_map = ConcurrentSkiplist::<i32, i32>::new(
        ConcurrentSkiplistMode::NoLock
    );

    // 批量插入数据
    for key in 1..=10000 {
        let value = rng.gen_range(-10000..=10000);
        std_map.insert(key, value);
        our_map.insert_or_update(key, value);
    }

    // 生成将被删除的 key
    let mut keys = Vec::<i32>::new();

    loop {
        let item = rng.gen_range(1..=10000);

        if !keys.contains(&item) {
            keys.push(item);
        }

        if keys.len() == 1000 {
            break;
        }
    }

    // 删除
    for key in keys {
        std_map.remove(&key);
        our_map.delete(&key, &(key + 1));
    }

    // 验证 ConcurrentSkiplist 内容
    for key in std_map.keys() {
        let wanted = std_map.get(&key);

        let will_be_none = our_map.get(&key, &key);
        assert_eq!(will_be_none.len(), 0);

        let will_be_single = our_map.get(&key, &(key + 1));
        assert_eq!(will_be_single.len(), 1);
        assert_eq!(will_be_single[0], wanted.unwrap());
    }

    // TODO 使用删除后的 key，our_map.get 将会返回什么？
}

// TODO 多个线程同时插入（每个线程的key不冲突），验证结果
// TODO 多个线程同时删除（每个线程key不冲突），验证结果
// TODO 待补充

#[test]
fn single_thread() {
    let map = Arc::new(ConcurrentSkiplist::<i32, i32>::new(
        ConcurrentSkiplistMode::NoLock
    ));
    for i in 1..2 {
        let map_ = map.clone();
        thread::spawn(move || {
            for j in i * 1000..(i + 1) * 1000 {
                map_.insert_or_update(j, j);
            }
            for j in i * 1000..(i + 1) * 1000 {
                let end = j + 1;
                let v = map_.get(&j, &end);
                assert_eq!(v.len(), 1);
                assert_eq!(*v[0], j);
            }
        });
    }
}

#[test]
fn multithread() {
    let map = Arc::new(ConcurrentSkiplist::<i32, i32>::new(
        ConcurrentSkiplistMode::NoLock
        // ConcurrentSkiplistMode::OneBigLock
    ));
    for i in 1..10 {
        let map_ = map.clone();
        thread::spawn(move || {
            let time=10000000;
            for j in i * time..(i + 1) * time {
                map_.insert_or_update(j, j);
            }
            for j in i * time..(i + 1) * time {
                let end = j + 1;
                let v = map_.get(&j, &end);
                assert_eq!(v.len(), 1);
                assert_eq!(*v[0], j);
            }
        });
    }
}
