use concurrent_skiplist::lib3::SkipListjjj;
use concurrent_skiplist::IndexOperate;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use crossbeam_skiplist::SkipMap;
use parking_lot::Mutex;
use rand::Rng;
use std::ops::Range;
use std::vec;
use std::{collections::BTreeMap, sync::Arc, thread};

pub fn bench_write_order(c: &mut Criterion) {
    let cnt=12500;
    let thread_cnt=10;
    c.bench_function("顺序写效率测试: BTreeMap", |b| {
        b.iter(|| {
            let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
            let mut vec = vec![];

            for i in 0 .. thread_cnt{
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i*cnt..(i+1)*cnt {
                        map_1.lock().insert(j, j);
                    }
                }));
            }
            // let m = 0..10000;
            // let n = 10000..20000;
            for i in vec {
                i.join().unwrap();
            }
        })
    });

    c.bench_function("顺序写效率测试: SkipListjjj", |b| {
        b.iter(|| {
            let map = Arc::new(SkipListjjj::<i32, i32>::new());

            let mut vec = vec![];

            for i in 0 .. thread_cnt{
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i*cnt..(i+1)*cnt {
                        map_1.insert_or_update(j, j);
                    }
                }));
            }
            // let m = 0..10000;
            // let n = 10000..20000;
            for i in vec {
                i.join().unwrap();
            }
        })
    });
}
pub fn bench_write_radom(c: &mut Criterion) {
    c.bench_function("随机写效率测试: BTreeMap", |b| {
        // 为随机写准备数据
        let mut rng = rand::thread_rng();
        let (mut keys_BTree_1, mut keys_BTree_2) = ([0; 10000], [0; 10000]);
        let (mut len_1, mut len_2) = (0, 0);
        loop {
            let item = rng.gen_range(1..20000);
            if !keys_BTree_1.contains(&item) {
                keys_BTree_1[len_1] = item;
                len_1 += 1;
            }
            if len_1 == 10000 {
                break;
            }
        }
        loop {
            let item = rng.gen_range(20001..40000);
            if !keys_BTree_2.contains(&item) {
                keys_BTree_2[len_2] = item;
                len_2 += 1;
            }
            if len_2 == 10000 {
                break;
            }
        }

        b.iter(move || {
            let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));

            let mut vec = vec![];
            let map_1 = map.clone();
            let map_2 = map.clone();

            vec.push(thread::spawn(move || {
                for i in keys_BTree_1 {
                    map_1.lock().insert(i, i);
                }
            }));
            vec.push(thread::spawn(move || {
                for i in keys_BTree_2 {
                    map_2.lock().insert(i, i);
                }
            }));
            for i in vec {
                i.join().unwrap();
            }
        })
    });

    c.bench_function("随机写效率测试: SkipListjjj", |b| {
        // 为随机写准备数据
        let mut rng = rand::thread_rng();
        let (mut keys_jjj_1, mut keys_jjj_2) = ([0; 10000], [0; 10000]);
        let (mut len_1, mut len_2) = (0, 0);
        loop {
            let item = rng.gen_range(1..20000);
            if !keys_jjj_1.contains(&item) {
                keys_jjj_1[len_1] = item;
                len_1 += 1;
            }
            if len_1 == 10000 {
                break;
            }
        }
        loop {
            let item = rng.gen_range(20001..40000);
            if !keys_jjj_2.contains(&item) {
                keys_jjj_2[len_2] = item;
                len_2 += 1;
            }
            if len_2 == 10000 {
                break;
            }
        }

        b.iter(move || {
            let map = Arc::new(SkipListjjj::<i32, i32>::new());

            let mut vec = vec![];
            let map_1 = map.clone();
            let map_2 = map.clone();

            vec.push(thread::spawn(move || {
                for i in keys_jjj_1 {
                    map_1.insert_or_update(i, i);
                }
            }));
            vec.push(thread::spawn(move || {
                for i in keys_jjj_2 {
                    map_2.insert_or_update(i, i);
                }
            }));
            for i in vec {
                i.join().unwrap();
            }
        })
    });
}

pub fn bench_read_small(c: &mut Criterion) {
    c.bench_function("少量范围读效率测试: BTreeMap", |b| {
        // 为少量读准备数据
        let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
        let m = 1..10000;
        for i in m {
            map.lock().insert(i, i);
        }

        b.iter(move || {
            // 少量读
            
        })
    });

    c.bench_function("少量范围读效率测试: SkipListjjj",  |b| {
        // 为少量读准备数据
        let map = Arc::new(SkipListjjj::<i32, i32>::new());
        let m = 1..10000;
        for i in m {
            map.insert_or_update(i, i);
        }

        b.iter(move || {
            // 少量读
        })
    });
}

pub fn bench_read_huge(c: &mut Criterion) {
    c.bench_function("大量范围读效率测试: BTreeMap", |b| {
        // 为大量读准备数据
        let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
        let m = 1..10000;
        for i in m {
            map.lock().insert(i, i);
        }

        b.iter(move || {
            // 大量读
        })
    });

    c.bench_function("大量范围读效率测试: SkipListjjj",  |b| {
        // 为大量读准备数据
        let map = Arc::new(SkipListjjj::<i32, i32>::new());
        let m = 1..10000;
        for i in m {
            map.insert_or_update(i, i);
        }

        b.iter(move || {
            // 大量读
        })
    });
}

criterion_group!(
    benches,
    bench_write_order,
<<<<<<< HEAD
    // bench_write_radom,
    // bench_read_order,
    // bench_read_radom
=======
    bench_write_radom,
    // bench_read_small,
    // bench_read_huge,
>>>>>>> origin/lock_free
);

criterion_main!(benches);
