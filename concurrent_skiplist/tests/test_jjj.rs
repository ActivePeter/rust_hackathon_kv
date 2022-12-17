use concurrent_skiplist::{IndexOperate};
use rand::Rng;
use std::{collections::BTreeMap, sync::Arc, thread};
use concurrent_skiplist::lib3::SkipListjjj;
/// cargo test <func_name> --test <file_name> -- --show-output
/// e.g. cargo test test_insert_batch --test test -- --show-output
#[test]
/// 单线程批量插入
fn test_insert_batch() {
    // prepare
    let mut rng = rand::thread_rng();
    let mut std_map = BTreeMap::<i32, i32>::new();
    let our_map = Arc::new(SkipListjjj::<i32, i32>::new());

    // insert
    for key in 1..=10000 {
        let value = rng.gen_range(-10000..=10000);
        std_map.insert(key, value);
        our_map.insert_or_update(key, value);
    }

    // checkout
    for key in 1..=10000 {
        let want = std_map.get(&key);

        let len_0 = our_map.get(&key, &key);
        assert_eq!(len_0.len(), 0);

        let len_1 = our_map.get(&key, &(key + 1));
        assert_eq!(len_1.len(), 1);
        assert_eq!(len_1[0], want.unwrap());
    }
}

#[test]
/// 测试重复插入
fn test_insert_repeat() {
    let our_map = Arc::new(SkipListjjj::<i32, i32>::new());
    let (key, range_end) = (1, 3);
    let val_1 = 1;
    let val_2 = 2;

    our_map.insert_or_update(key, val_1);
    our_map.insert_or_update(key, val_2);

    let got = our_map.get(&key, &range_end);
    assert_eq!(got.len(), 1, "len is wrong");
    assert_eq!(*got[0], 2, "val isn't the last insert val");
}

#[test]
/// 测试获取不存在的值
fn test_get_non_exist() {
    let our_map = Arc::new(SkipListjjj::<i32, i32>::new());
    let (key, range_end) = (3,7);
    let (key_left,val_left) = (1,1);
    let (key_right,val_right) = (7,7);

    our_map.insert_or_update(key_left, val_left);
    our_map.insert_or_update(key_right, val_right);

    // 原本应取得 3<=key<7 之间的集合
    // 但是此处不存在对应的取值
    // 因此 数组长度为 0 
    let got = our_map.get(&key, &range_end);
    assert_eq!(got.len(),0,"len is wrong")
}

#[test]
/// 单线程批量删除
fn test_delete_batch() {
    // 初始化
    let mut rng = rand::thread_rng();
    let mut std_map = BTreeMap::<i32, i32>::new();
    let our_map = Arc::new(SkipListjjj::<i32, i32>::new());

    // 批量插入10条数据
    for key in 1..=10 {
        std_map.insert(key, 10000-key);
        our_map.insert_or_update(key, 10000-key);
    }

    // 随机生成 9个用于被删除的 key
    let mut keys = Vec::<i32>::new();
    loop {
        let item = rng.gen_range(1..=10);
        if !keys.contains(&item) {
            keys.push(item);
        }
        if keys.len() == 9 {
            break;
        }
    }

    // 删除
    for key in keys {
        std_map.remove(&key);
        our_map.delete(&key, &(key + 1));
    }

    // 未删除的key(仅剩一个)
    for key in std_map.keys() {
        let want = std_map.get(&key);
        let got = our_map.get(&key, &(key + 1));
        // println!("------std--------\nkey:{}\ngot:{}",key,want.unwrap());
        // println!("------our--------\nkey:{}\ngot:{}",key,got[0]);
        assert_eq!(got[0], want.unwrap());
    }
}

#[test]
/// 测试获取删除后的值
fn test_get_after_delete(){
    let our_map = Arc::new(SkipListjjj::<i32, i32>::new());
    let (key, val,range_end) = (3,99,4);

    our_map.insert_or_update(key, val);

    our_map.delete(&key, &range_end);

    let got = our_map.get(&key, &range_end);
    assert_eq!(got.len(),0,"len is wrong, maybe not be delete really")
}
// TODO 多个线程同时插入（每个线程key不冲突），验证结果
// TODO 多个线程同时删除（每个线程key不冲突），验证结果
// TODO 待补充

#[test]
fn single_thread() {
    // let map = Arc::new(ConcurrentSkiplist::<i32, i32>::new(
    //     ConcurrentSkiplistMode::NoLock,
    // ));
    // for i in 1..2 {
    //     let map_ = map.clone();
    //     thread::spawn(move || {
    //         for j in i * 1000..(i + 1) * 1000 {
    //             map_.insert_or_update(j, j);
    //         }
    //         for j in i * 1000..(i + 1) * 1000 {
    //             let end = j + 1;
    //             let v = map_.get(&j, &end);
    //             assert_eq!(v.len(), 1);
    //             assert_eq!(*v[0], j);
    //         }
    //     });
    // }
}

#[test]
fn multithread() {
    // let map = Arc::new(ConcurrentSkiplist::<i32, i32>::new(
    //     ConcurrentSkiplistMode::NoLock, // ConcurrentSkiplistMode::OneBigLock
    // ));
    // let mut v = vec![];
    // for i in 1..2 {
    //     let map_ = map.clone();
    //     v.push(thread::spawn(move || {
    //         let time = 10000;
    //         for j in i * time..(i + 1) * time {
    //             map_.insert_or_update(j, j);
    //         }
    //         for j in i * time..(i + 1) * time {
    //             let end = j + 1;
    //             let v = map_.get(&j, &end);
    //             assert_eq!(v.len(), 1);
    //             assert_eq!(*v[0], j);
    //         }
    //     }));
    // }
    // for u in v {
    //     u.join().unwrap();
    // }
}
