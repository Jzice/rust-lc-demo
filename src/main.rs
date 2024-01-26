use dashmap::DashMap;
use std::sync::{Arc, RwLock};

mod ds;
mod alg; 

fn main() {
    let my_map: Arc<RwLock<DashMap<String, i32>>> = Arc::new(RwLock::new(DashMap::new()));
    my_map.write().unwrap().insert("a".to_string(), 1);
    my_map.write().unwrap().insert("b".to_string(), 2);

    let my_map_clone = Arc::clone(&my_map);
    let read_lock = my_map_clone.read().unwrap();

    // 迭代读者锁上的 DashMap
    for item in read_lock.iter() {
        println!("key: {}, value: {}", item.key(), item.value());
    }
}
