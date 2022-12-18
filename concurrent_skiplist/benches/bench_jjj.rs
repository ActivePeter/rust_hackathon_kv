use concurrent_skiplist::lib3::SkipListjjj;
use concurrent_skiplist::IndexOperate;
// use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion::{criterion_group, criterion_main, Criterion};
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

pub fn bench_wr_read_order_1(c: &mut Criterion) {
    c.bench_function("多线程分范围读: BTreeMap", |b| {
        let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
        for i in 0..READ_WR_THREAD * READ_WR_CAPACITY {
            map.lock().insert(i, i);
        }
        b.iter(move|| {
            let mut vec = vec![];
            for i in 0..READ_WR_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i * READ_WR_CAPACITY..(i + 1) * READ_WR_CAPACITY {
                        map_1.lock().get(&j);
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });

    c.bench_function("多线程分范围读: SkipListjjj", |b| {
        let map = Arc::new(SkipListjjj::<i32, i32>::new());
        for i in 0..READ_WR_THREAD * READ_WR_CAPACITY {
            map.insert_or_update(i, i);
        }
        b.iter(|| {
            let mut vec = vec![];
            for i in 0..READ_WR_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in i * READ_WR_CAPACITY..(i + 1) * READ_WR_CAPACITY {
                        map_1.get(&j, &(j + 1));
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
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

pub fn bench_read_small(c: &mut Criterion) {
    c.bench_function("少量范围读效率测试: BTreeMap", |b| {
        // 为少量读准备数据
        let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
        let m = 0..READ_SMALL_CAPACITY * READ_SMALL_THREAD;
        for i in m {
            map.lock().insert(i, i);
        }

        b.iter(move || {
            // 少量读
            let mut vec = vec![];
            for i in 0..READ_SMALL_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in 0..READ_SMALL_CAPACITY / READ_SMALL {
                        let _p = map_1.lock().range(
                            j + i * READ_SMALL_CAPACITY..j + READ_SMALL + i * READ_SMALL_CAPACITY,
                        );
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });

    c.bench_function("少量范围读效率测试: SkipListjjj", |b| {
        // 为少量读准备数据
        let map = Arc::new(SkipListjjj::<i32, i32>::new());
        let m = 0..READ_SMALL_CAPACITY * READ_SMALL_THREAD;
        for i in m {
            map.insert_or_update(i, i);
        }

        b.iter(move || {
            // 少量读
            let mut vec = vec![];
            for i in 0..READ_SMALL_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in 0..READ_SMALL_CAPACITY / READ_SMALL {
                        let _p = map_1.get(
                            &(j + i * READ_SMALL_CAPACITY),
                            &(j + READ_SMALL + i * READ_SMALL_CAPACITY),
                        );
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });
}

pub fn bench_read_huge(c: &mut Criterion) {
    c.bench_function("大量范围读效率测试: BTreeMap", |b| {
        // 为大量读准备数据
        let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
        let m = 0..READ_HUGE_CAPACITY * READ_HUGE_THREAD;
        for i in m {
            map.lock().insert(i, i);
        }

        b.iter(move || {
            // 大量读
            let mut vec = vec![];
            for i in 0..READ_HUGE_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in 0..READ_HUGE_CAPACITY / READ_HUGE {
                        let _p = map_1.lock().range(
                            j + i * READ_HUGE_CAPACITY..j + READ_HUGE + i * READ_HUGE_CAPACITY,
                        );
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });

    c.bench_function("大量范围读效率测试: SkipListjjj", |b| {
        // 为大量读准备数据
        let map = Arc::new(SkipListjjj::<i32, i32>::new());
        let m = 0..READ_HUGE_CAPACITY * READ_HUGE_THREAD;
        for i in m {
            map.insert_or_update(i, i);
        }

        b.iter(move || {
            // 大量读
            let mut vec = vec![];
            for i in 0..READ_HUGE_THREAD {
                let map_1 = map.clone();
                vec.push(thread::spawn(move || {
                    for j in 0..READ_HUGE_CAPACITY / READ_HUGE {
                        let _p = map_1.get(
                            &(j + i * READ_HUGE_CAPACITY),
                            &(j + READ_HUGE + i * READ_HUGE_CAPACITY),
                        );
                    }
                }));
            }
            for i in vec {
                i.join().unwrap();
            }
        })
    });
}

criterion_group!(
    benches,
    bench_write_order,
    bench_wr_read_order_1,
    bench_write_radom,
    bench_read_small,
    bench_read_huge,
);

criterion_main!(benches);
