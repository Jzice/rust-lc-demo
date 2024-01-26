// lru.rs
//

use std::collections::VecDeque;
use std::collections::HashMap;

type LRUKey = i32;
type LRUVal = i32;

/// Last Recent Used Cache
struct LRUCache {
    n: usize,                               // 缓存容量
    cache_map: HashMap<LRUKey, LRUVal>,     // k-v 哈希表 
    keys: VecDeque<LRUKey>,                 // 按key访问先后存储的双端队列
}

/// LRUCache 操作
impl LRUCache {
    pub fn new(n: usize) -> Self {
        LRUCache {
            n,
            cache_map: HashMap::new(),
            keys: VecDeque::new(),
        }
    }
    
    /// 读操作
    pub fn get(&mut self, key: LRUKey) -> Option<LRUVal> {
        self.cache_map.get(&key).map(|&v|{
            self.keys.iter().position(|&k| k == key)
                .map(|i|self.keys.remove(i));
            self.keys.push_front(key);
            v
        }) 
    }

    /// 写操作
    pub fn put(&mut self, k: LRUKey, v: LRUVal) {
        match self.cache_map.get(&k) {
            None => {
                if self.is_full() {
                    // 删除最旧元素
                    self.keys.pop_back()
                        .map(|invalid_key| {
                            self.cache_map.remove(&invalid_key)
                        });
                }
                self.cache_map.insert(k, v);
                self.keys.push_front(k);
            }
            Some(_) => {
                self.keys.iter()
                    .position(|&k_| k_ == k)
                    .map(|i|self.keys.remove(i));
                self.keys.push_front(k);
                self.cache_map.insert(k, v);
            }
        }
    }

    fn is_full(&self) -> bool {
        self.n == self.keys.len()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut lru = LRUCache::new(10);

        assert!(!lru.is_full());
        assert!(lru.get(5) == None);
        for i in 0..10 {
            lru.put(i, i);
        }
        assert!(lru.is_full());
        assert!(lru.get(5) == Some(5));
        assert!(lru.get(15) == None);
    }
}
