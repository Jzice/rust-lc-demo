// lfu.rs
//
//

use std::collections::HashMap;
use std::collections::BTreeSet;

type LFUKey = i32;
type LFUVal = i32;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LFUEntry {
    freq: usize,    // 访问计数
    time: usize,    // 初始时间, 当freq相同时, 先生成的entry先被淘汰
    key: LFUKey,
    val: LFUVal,
}

/// LFU缓存
struct LFUCache {
    n: usize,       // cache容量
    time: usize,    // 更新时间
    cache_map: HashMap<LFUKey, LFUEntry>,  // 哈希表
    keys: BTreeSet<LFUEntry>, // 有序列表
}

impl LFUCache {
    pub fn new(n: usize) -> Self {
        LFUCache {
            n,
            time: 0,
            cache_map: HashMap::new(),
            keys: BTreeSet::new(),
        }
    }

    fn is_full(&self) -> bool {
        self.cache_map.len() == self.n
    }

    /// 插入k,v
    pub fn put(&mut self, k: LFUKey, v: LFUVal) {
        self.time += 1;
        match self.cache_map.get_mut(&k) {
            Some(entry) => {
                // 删除旧key
                self.keys.remove(&entry);

                // 更新entry
                entry.freq += 1;
                entry.val = v;

                // 重新排序
                self.keys.insert(entry.clone());
            }
            None => {
                // 如果cache full
                if self.is_full() {
                    // 移除最使用的数据
                    self.keys.pop_first().map(|oldest_entry|{
                        self.cache_map.remove(&oldest_entry.key);
                    });
                }
                // 更新entry
                let entry = LFUEntry {
                    freq: 0,
                    time: self.time,
                    key: k,
                    val: v,
                };
                self.keys.insert(entry.clone());
                self.cache_map.insert(k, entry);
            }
        }
    }

    /// 查询操作
    pub fn get(&mut self, k: LFUKey) -> Option<LFUVal> {
        self.cache_map.get_mut(&k).map(|entry| {
            self.keys.remove(&entry);
            entry.freq += 1;                   // 更新entry计数
            self.keys.insert(entry.clone());    // 更新key排序

            entry.val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut lfu1 = LFUCache::new(3);

        assert!(!lfu1.is_full());
        assert!(lfu1.get(1) == None);

        for i in (0..3).rev() {
            lfu1.put(i, i)
        }
        assert!(lfu1.is_full());
        assert!(lfu1.get(0) == Some(0));
        assert!(lfu1.get(1) == Some(1));
        assert!(lfu1.get(2) == Some(2));
        assert!(lfu1.get(3) == None);
        lfu1.put(10, 10);
        assert!(lfu1.is_full());
        assert!(lfu1.cache_map.len() == 3);
        assert!(lfu1.get(0) == Some(0));
        assert!(lfu1.get(1) == Some(1));
        assert!(lfu1.get(2) == None);   
        assert!(lfu1.get(10) == Some(10));
    }

}
