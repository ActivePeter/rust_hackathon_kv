use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use concurrent_skiplist::{ConcurrentSkiplist, ConcurrentSkiplistMode, IndexOperate};
use rand::Rng;
use std::{collections::BTreeMap, sync::Arc, thread};
use crossbeam_skiplist::SkipMap;
use parking_lot::Mutex;
// use criterion::async_executor::FuturesExecutor;

fn criterion_benchmark(c: &mut Criterion) {
    // let pakv=PaKVCtx::create();
    let mut i=0;
    // c.bench_function("one thread no", |b| b.iter(|| {
    //     let map = Arc::new(ConcurrentSkiplist::<i32, i32>::new(
    //         ConcurrentSkiplistMode::NoLock
    //         // ConcurrentSkiplistMode::OneBigLock
    //     ));
    //     let mut vec =vec![];
    //     for i in 1..2 {
    //         let map_ = map.clone();
    //         vec.push(thread::spawn(move || {
    //             let time=10000;
    //             for j in i * time..(i + 1) * time {
    //                 map_.insert_or_update(j, j);
    //             }
    //             for j in i * time..(i + 1) * time {
    //                 let end = j + 1;
    //                 let v = map_.get(&j, &end);
    //                 assert_eq!(v.len(), 1);
    //                 assert_eq!(*v[0], j);
    //             }
    //         }));
    //     }
    //     for i in vec {
    //         i.join().unwrap();
    //     }
    //     // pakv.set(format!("{}",i),"lll".to_string());
    //     // // pakv.set("hhh".to_string(),"mmm".to_string());
    //     // i=i+1;
    // }));
    // c.bench_function("1 thread cross", |b| b.iter(|| {
    //     let map = Arc::new(SkipMap::<i32, i32>::new(
    //         // ConcurrentSkiplistMode::NoLock
    //         // ConcurrentSkiplistMode::OneBigLock
    //     ));
    //     let mut vec =vec![];
    //     for i in 1..2 {
    //         let map_ = map.clone();
    //         vec.push(thread::spawn(move || {
    //             let time=10000;
    //             for j in i * time..(i + 1) * time {
    //                 map_.insert(j, j);
    //             }
    //             // for k in 0..100
    //             {
    //                 for j in i * time..(i + 1) * time {
    //                     let end = j + 1;
    //                     // let v = ;//get(&j, &end);
    //                     let mut v =vec![];
    //                     let r=map_.range(j..end);
    //                     // let lock=map_.lock();
    //                     for x in r{
    //                         v.push(*x.value());
    //                     };
    //                     assert_eq!(v.len(), 1);
    //                     assert_eq!(v[0], j);
    //                 }
    //             }
    //
    //         }));
    //     }
    //     for i in vec {
    //         i.join().unwrap();
    //     }
    //     // pakv.set(format!("{}",i),"lll".to_string());
    //     // // pakv.set("hhh".to_string(),"mmm".to_string());
    //     // i=i+1;
    // }));
    // c.bench_function("one thread btree", |b| b.iter(|| {
    //     let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new(
    //         // ConcurrentSkiplistMode::NoLock
    //         // ConcurrentSkiplistMode::OneBigLock
    //     )));
    //     let mut vec =vec![];
    //     for i in 1..2 {
    //         let mut map_ = map.clone();
    //         vec.push(thread::spawn(move || {
    //             let mut map_ =map_.lock();
    //             let time=10000;
    //             for j in i * time..(i + 1) * time {
    //                 map_.insert(j, j);
    //             }
    //
    //                 for j in i * time..(i + 1) * time {
    //                     let end = j + 1;
    //                     // map_.range()
    //                     let mut v =vec![];
    //                     for x in map_.range(j..end) {
    //                         v.push(x.1);
    //                     };
    //                     assert_eq!(v.len(), 1);
    //                     assert_eq!(*v[0], j);
    //                 }
    //
    //         }));
    //     }
    //     for i in vec {
    //         i.join().unwrap();
    //     }
    //     // pakv.set(format!("{}",i),"lll".to_string());
    //     // // pakv.set("hhh".to_string(),"mmm".to_string());
    //     // i=i+1;
    // }));
    //
    // c.bench_function("one thread big", |b| b.iter(|| {
    //     let map = Arc::new(ConcurrentSkiplist::<i32, i32>::new(
    //         // ConcurrentSkiplistMode::NoLock
    //         ConcurrentSkiplistMode::OneBigLock
    //     ));
    //     let mut vec =vec![];
    //     for i in 1..2 {
    //         let map_ = map.clone();
    //         vec.push(thread::spawn(move || {
    //             let time=10000;
    //             for j in i * time..(i + 1) * time {
    //                 map_.insert_or_update(j, j);
    //             }
    //             for j in i * time..(i + 1) * time {
    //                 let end = j + 1;
    //                 let v = map_.get(&j, &end);
    //                 assert_eq!(v.len(), 1);
    //                 assert_eq!(*v[0], j);
    //             }
    //         }));
    //     }
    //     for i in vec {
    //         i.join().unwrap();
    //     }
    //     // pakv.set(format!("{}",i),"lll".to_string());
    //     // // pakv.set("hhh".to_string(),"mmm".to_string());
    //     // i=i+1;
    // }));
    c.bench_function("8 thread cross wr", |b| b.iter(|| {
        let map = Arc::new(SkipMap::<i32, i32>::new(
            // ConcurrentSkiplistMode::NoLock
            // ConcurrentSkiplistMode::OneBigLock
        ));
        let mut vec =vec![];
        for i in 1..9 {
            let map_ = map.clone();
            vec.push(thread::spawn(move || {
                let time=12500;
                for j in i * time..(i + 1) * time {
                    map_.insert(j, j);
                }
                // for k in 0..100{
                //     for j in i * time..(i + 1) * time {
                //         let end = j + 1;
                //         // let v = ;//get(&j, &end);
                //         let mut v =vec![];
                //         let r=map_.range(j..end);
                //         // let lock=map_.lock();
                //         for x in r{
                //             v.push(*x.value());
                //         };
                //         assert_eq!(v.len(), 1);
                //         assert_eq!(v[0], j);
                //     }
                // }

            }));
        }
        for i in vec {
            i.join().unwrap();
        }
        // pakv.set(format!("{}",i),"lll".to_string());
        // // pakv.set("hhh".to_string(),"mmm".to_string());
        // i=i+1;
    }));
    c.bench_function("8 thread my wr", |b| b.iter(|| {
        let map = Arc::new(ConcurrentSkiplist::<i32, i32>::new(
            ConcurrentSkiplistMode::NoLock
            // ConcurrentSkiplistMode::OneBigLock
        ));
        let mut vec =vec![];
        for i in 1..9 {
            let map_ = map.clone();
            vec.push(thread::spawn(move || {
                let time=12500;
                for j in i * time..(i + 1) * time {
                    map_.insert_or_update(j, j);
                }
                // for k in 0..100{
                //     for j in i * time..(i + 1) * time {
                //         let end = j + 1;
                //         let v = map_.get(&j, &end);
                //         assert_eq!(v.len(), 1);
                //         assert_eq!(*v[0], j);
                //     }
                // }

            }));
        }
        for i in vec {
            i.join().unwrap();
        }
        // pakv.set(format!("{}",i),"lll".to_string());
        // // pakv.set("hhh".to_string(),"mmm".to_string());
        // i=i+1;
    }));
    c.bench_function("8 thread btree wr", |b| b.iter(|| {
        let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new(
            // ConcurrentSkiplistMode::NoLock
            // ConcurrentSkiplistMode::OneBigLock
        )));
        let mut vec =vec![];
        for i in 1..9 {
            let mut map_ = map.clone();
            vec.push(thread::spawn(move || {
                // let mut map_ =map_.lock();
                let time=12500;
                for j in i * time..(i + 1) * time {
                    map_.lock().insert(j, j);
                }
                // for k in 0..100{
                //     for j in i * time..(i + 1) * time {
                //         let end = j + 1;
                //         // map_.range()
                //         {
                //             let mut v =vec![];
                //             let lock=map_.lock();
                //             for x in lock.range(j..end) {
                //                 v.push(x.1);
                //             };
                //             assert_eq!(v.len(), 1);
                //             assert_eq!(*v[0], j);
                //         }
                //     }
                // }
            }));
        }
        for i in vec {
            i.join().unwrap();
        }
        // pakv.set(format!("{}",i),"lll".to_string());
        // // pakv.set("hhh".to_string(),"mmm".to_string());
        // i=i+1;
    }));
    // c.bench_function("get exist", |b| b.iter(|| {
    //     pakv_chan_handler.get("kskskskk".to_string());
    // }));
}

// fn criterion_benchmark2(c: &mut Criterion) {
//     // env_logger::init();//remember to set RUST_LOG=INFO
//     let pakv_chan_handler=pakv::start_kernel();
//     pakv_chan_handler.del("lalala".to_string());
//     pakv_chan_handler.set("ksksksk".to_string(),"sss".to_string());
//
//     c.bench_function("del not exist, set", |b| b.iter(|| {
//         pakv_chan_handler.del("lalala".to_string());
//         pakv_chan_handler.set("ksksksk".to_string(),"sss".to_string());
//     }));
//     c.bench_function("del exist, set", |b| b.iter(|| {
//         pakv_chan_handler.del("ksksksk".to_string());
//         pakv_chan_handler.set("ksksksk".to_string(),"sss".to_string());
//     }));
//     c.bench_function("set", |b| b.iter(|| {
//         // pakv_chan_handler.del("mmm".to_string());
//         pakv_chan_handler.set("mmm".to_string(),"sss".to_string());
//     }));
// }

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);