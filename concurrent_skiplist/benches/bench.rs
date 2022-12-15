use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BTreeMap;

#[inline]
fn init(map: &mut BTreeMap<i32, i32>) {
    for number in 1..100000 {
        map.insert(number, number);
    }
}

#[inline]
fn get(map: &mut BTreeMap<i32, i32>) {
    for number in 1..100000 {
        map.get(&number);
    }

    for number in (1..100000).rev() {
        map.get(&number);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut map = BTreeMap::<i32, i32>::new();
    init(&mut map);

    c.bench_function("get", |b| b.iter(|| get(&mut map)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
