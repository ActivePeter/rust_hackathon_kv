use concurrent_skiplist::skiplist::SkipListjjj;
use concurrent_skiplist::IndexOperate;
// use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion::{criterion_group, criterion_main, Criterion, Bencher};
use parking_lot::Mutex;
use rand::Rng;
use std::vec;
use std::{collections::BTreeMap, sync::Arc, thread};

// 顺序写测试数据容量
// 顺序写测试线程数
const WRITE_ORDER_CAPACITY: i32 = 12500;
const WRITE_ORDER_THREAD: i32 = 10;

// 分范围读测试数据容量
// 分范围读试线程数
const READ_WR_CAPACITY: i32 = 12500;
const READ_WR_THREAD: i32 = 10;

// 随机写测试数据容量
// 随机写测试线程数
const WRITE_RADOM_CAPACITY: i32 = 12500;
const WRITE_RADOM_THREAD: i32 = 10;

// 小范围读取测试容量
// 小范围读取规模(一次性读多少数据)
const READ_SMALL_CAPACITY: i32 = 12500;
const READ_SMALL_THREAD: i32 = 10;
const READ_SMALL: i32 = 100;

// 大范围读取测试容量
// 大范围读取规模(一次性读多少数据)
const READ_HUGE_CAPACITY: i32 = 12500;
const READ_HUGE_THREAD: i32 = 10;
const READ_HUGE: i32 = 1000;

pub fn bench_write_order(c: &mut Criterion) {
    c.bench_function("顺序写效率测试: BTreeMap", |b| {
        b.iter(|| {
            let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
            let mut vec = vec![];

            for i in 0..WRITE_ORDER_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i * WRITE_ORDER_CAPACITY..(i + 1) * WRITE_ORDER_CAPACITY {
                        map_1.lock().insert(j, j);
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });

    c.bench_function("顺序写效率测试: SkipListjjj", |b| {
        b.iter(|| {
            let map = Arc::new(SkipListjjj::<i32, i32>::new());

            let mut vec = vec![];

            for i in 0..WRITE_ORDER_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i * WRITE_ORDER_CAPACITY..(i + 1) * WRITE_ORDER_CAPACITY {
                        map_1.insert_or_update(j, j);
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });
}


fn threads_read_btree(range:i32,b:&mut Bencher){
    let thread_cnt=10;
    // 为大量读准备数据
    let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
    let m = 0..range * thread_cnt;//10个线程
    for i in m {
        map.lock().insert(i, i);
    }
    b.iter(move || {
        // 大量读
        let mut vec = vec![];
        for i in 0..thread_cnt {
            let map_1 = map.clone();
            vec.push(thread::spawn(move || {

                for _ in 0..100 {
                    let mut reses =vec![];
                    for x in map_1.lock().range(i * range..(i + 1) * range) {
                        reses.push(x.1);
                    }
                }

                // for j in i*range..(i+1)*range {
                //     let res=map_1.lock().remove(&j);
                //     reses.push(res.unwrap());
                // }
            }));
        }
        for i in vec {
            i.join().unwrap();
        }
    })
}
fn threads_read_skiplist(range:i32,b:&mut Bencher){
    let thread_cnt=10;
    // 为大量读准备数据
    let map = Arc::new(SkipListjjj::<i32, i32>::new());
    let m = 0..range * thread_cnt;//10个线程
    for i in m {
        map.insert_or_update(i, i);
    }
    b.iter(move || {
        let mut vec = vec![];
        for i in 0..thread_cnt {
            let map_1 = map.clone();
            vec.push(thread::spawn(move || {
                for _ in 0..100 {
                    assert_eq!(map_1.get(&(i*range),&((i+1)*range)).len(),range as usize);
                }
                // let mut reses =vec![];

                // // for j in i*range..(i+1)*range {
                // reses.push(map_1.delete(&(i*range),&((i+1)*range)))

            }));
        }
        for i in vec {
            i.join().unwrap();
        }
    })
}

pub fn bench_read(c: &mut Criterion) {
    c.bench_function("多线程分范围1读: BTreeMap", |b| {
        threads_read_btree(1,b);
    });

    c.bench_function("多线程分范围1读: SkipListjjj", |b| {
        threads_read_skiplist(1,b);
    });
    c.bench_function("多线程分范围100读: BTreeMap", |b| {
        threads_read_btree(100,b);
    });

    c.bench_function("多线程分范围100读: SkipListjjj", |b| {
        threads_read_skiplist(100,b);
    });

    c.bench_function("多线程分范围10000读: BTreeMap", |b| {
        threads_read_btree(10000,b);
    });

    c.bench_function("多线程分范围10000读: SkipListjjj", |b| {
        threads_read_skiplist(10000,b);
    });

    c.bench_function("多线程分范围100000读: BTreeMap", |b| {
        threads_read_btree(100000,b);
    });

    c.bench_function("多线程分范围100000读: SkipListjjj", |b| {
        threads_read_skiplist(100000,b);
    });
}

pub fn bench_write_radom(c: &mut Criterion) {
    // 为随机写准备数据
    let mut rng = rand::thread_rng();
    let mut keys = vec![];
    for i in 0..WRITE_RADOM_THREAD {
        let mut keys_ = [0; WRITE_RADOM_CAPACITY as usize];
        for j in 0..WRITE_RADOM_CAPACITY {
            let item = rng.gen_range(
                1 + i * WRITE_RADOM_CAPACITY * 10
                    ..WRITE_RADOM_CAPACITY * 10 + i * WRITE_RADOM_CAPACITY * 10,
            );
            keys_[j as usize] = item;
        }
        keys.push(keys_)
    }
    
    c.bench_function("随机写效率测试: BTreeMap", |b| {

        b.iter(|| {
            let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
            let mut vec = vec![];

            for &mut i in &mut keys {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i {
                        map_1.lock().insert(j, j);
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });

    c.bench_function("随机写效率测试: SkipListjjj", |b| {
        b.iter(|| {
            let map = Arc::new(SkipListjjj::<i32, i32>::new());
            let mut vec = vec![];

            for &mut i in &mut keys {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i {
                        map_1.insert_or_update(j, j);
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });
}




fn threads_delete_btree(range:i32,b:&mut Bencher){
    let thread_cnt=10;
    // 为大量读准备数据
    let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));

    b.iter(move || {
        let m = 0..range * thread_cnt;//10个线程
        for i in m {
            map.lock().insert(i, i);
        }
        // 大量读
        let mut vec = vec![];
        for i in 0..thread_cnt {
            let map_1 = map.clone();
            vec.push(thread::spawn(move || {
                let mut reses =vec![];
                for j in i*range..(i+1)*range {
                    let res=map_1.lock().remove(&j);
                    reses.push(res.unwrap());
                }
            }));
        }
        for i in vec {
            i.join().unwrap();
        }
    })
}
fn threads_delete_skiplist(range:i32,b:&mut Bencher){
    let thread_cnt=10;
    // 为大量读准备数据
    let map = Arc::new(SkipListjjj::<i32, i32>::new());

    b.iter(move || {
        let m = 0..range * thread_cnt;//10个线程
        for i in m {
            map.insert_or_update(i, i);
        }
        // 大量读
        let mut vec = vec![];
        for i in 0..thread_cnt {
            let map_1 = map.clone();
            vec.push(thread::spawn(move || {
                let mut reses =vec![];
                // for j in i*range..(i+1)*range {
                    reses.push(map_1.delete(&(i*range),&((i+1)*range)))

            }));
        }
        for i in vec {
            i.join().unwrap();
        }
    })
}

pub fn bench_delete(c: &mut Criterion) {
    c.bench_function("1范围删除效率测试: BTreeMap", |b| {
        threads_delete_btree(1,b);
    });
    c.bench_function("1范围删除效率测试: SkipListjjj", |b| {
        threads_delete_skiplist(1,b);
    });
    c.bench_function("100范围删除效率测试: BTreeMap", |b| {
        threads_delete_btree(100,b);
    });

    c.bench_function("100范围删除效率测试: SkipListjjj", |b| {
        threads_delete_skiplist(100,b);
    });
    c.bench_function("10000范围删除效率测试: BTreeMap", |b| {
        threads_delete_btree(10000,b);
    });

    c.bench_function("10000范围删除效率测试: SkipListjjj", |b| {
        threads_delete_skiplist(10000,b);
    });
    c.bench_function("100000范围删除效率测试: BTreeMap", |b| {
        threads_delete_btree(100000,b);
    });

    c.bench_function("100000范围删除效率测试: SkipListjjj", |b| {
        threads_delete_skiplist(100000,b);
    });
}

criterion_group!(
    benches,
    // bench_write_order,
    // bench_write_radom,
    // bench_read,
    bench_delete,
);

criterion_main!(benches);
