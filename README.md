# rust_hackathon_kv

## test todo

性能测试都和btree做下对比
- 写性能，
  - 多线程顺序写
  - 多线程随机写
- 读性能
  - （预先将数据写入，不计入测试
  - 多线程读不同范围的数据
    - 读分为小范围和大范围
- more 暂时没想到

## test record

### 正确性测试

test_insert_batch 测试插入后读取正确性

test_insert_repeat 测试插入重复值正确性

test_delete_batch 测试删除正确性

test_get_after_delete 测试是否会获取到删除后的值

### 性能测试

单线程性能不如btreemao
![img.png](rsc/test_1thread_wr.png)

单线程性能不如btreemao
![img.png](rsc/test_1thread_wr.png)