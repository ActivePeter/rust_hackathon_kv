use concurrent_skiplist::ConcurrentSkiplist;
fn main() {
    let map=ConcurrentSkiplist::<String,String>::new();
    //todo 启动几个线程，并行的往里塞数据，
    // 保证每个线程访问的key不同
}