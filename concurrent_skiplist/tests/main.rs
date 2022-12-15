use concurrent_skiplist::ConcurrentSkiplist;

#[test]
fn insert() {
    let map = ConcurrentSkiplist::<i32, i32>::new();

    for number in 1..1000 {
        map.insert(number, number);
    }
}
