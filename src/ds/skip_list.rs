//! # 跳表

use std::{rc::Rc, cell::RefCell};
use rand::Rng;

const LEVEL_MAX: usize = 32;
const P_FACTOR: f64 = 0.25;

type SkipLink = Option<Rc<RefCell<SkipNode>>>;

/// 跳表节点
pub struct SkipNode {
    val: i32,
    next: Vec<SkipLink>,
}

impl SkipNode {
    /// 新建SkipNode
    pub fn new(val: i32, level: usize) -> Self {
        Self {
            val,
            next: vec![None; level],
        }
    }
}

/// 生成随机level
fn get_rand_level() -> usize {
    let mut level = 1;
    let mut rng = rand::thread_rng();
    while level < LEVEL_MAX && rng.gen::<f64>() < P_FACTOR {
        level += 1;
    }

    level
}

/// 跳表
pub struct SkipList {
    /// 跳表最高层级 
    level: usize, 
    /// 跳表头
    head: SkipLink,   
}

impl SkipList {
    /// 新建跳表
    pub fn new() -> Self {
        Self {
            level: LEVEL_MAX,
            head: Some(Rc::new(RefCell::new(SkipNode::new(std::i32::MIN, LEVEL_MAX)))),
        }
    }

    /// 插入元素
    pub fn add(&mut self, val: i32)  {
        let level = get_rand_level();
        let new_node = Rc::new(RefCell::new(SkipNode::new(val, level)));

        let mut cur_link = self.head.clone();
        // 从level开始, 往下遍历各层
        for l in (0..level).rev() {
            // 从当前节点开始, 往后遍历
            while let Some(link_rc) = cur_link.clone() {
                let mut node = link_rc.borrow_mut();
                if node.val == val {
                    return;
                }
                // next节点val < 新节点val
                if let Some(next) = node.next[l].clone().filter(|n| n.borrow().val < val) {
                    // 往后移动当前指针
                    cur_link.replace(next);
                } else {
                    // 否则, 可以插入了
                    new_node.borrow_mut().next[l] = node.next[l].take();
                    // 将当前节点当前层级下一个节点指针 -> new_node
                    node.next[l].replace(new_node.clone());

                    // 中断当前层级遍历
                    break;
                }
            }
        }
    }

    /// 查找节点
    pub fn search(&self, val: i32) -> bool {
        let mut cur_link = self.head.clone();
        for l in (0..self.level).rev() {
            while let Some(link_rc) = cur_link.clone() {
                let node = link_rc.borrow();
                // 当前节点val == val
                if node.val == val {
                    return true; // 找到
                } 
                // 未找到， 继续找
                // 下个节点val <= val
                if let Some(next) = node.next[l].clone().filter(|n| n.borrow().val <= val) {
                    cur_link.replace(next);
                } else {
                    // 其他情况： 
                    // 1. 下一个节点val > val
                    // 2. 下一个节点为none
                    break;
                }
            }
        }

        false
    }

    /// 移除节点
    pub fn remove(&mut self, val: i32) -> bool {
        let mut exist = false;
        let mut cur_link = self.head.clone();
        for l in (0..self.level).rev() {
            while let Some(cur_link_rc) = cur_link.clone() {
                let mut node = cur_link_rc.borrow_mut();
                match node.next[l].clone() {
                    Some(next) if next.borrow().val == val => {
                        node.next[l] = next.borrow_mut().next[l].take();
                        exist = true;
                        break;
                    }
                    Some(next) if next.borrow().val < val => {
                        cur_link.replace(next);
                    }
                    _ => break,
                }
            }
        }

        exist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut sl1 = SkipList::new();

        assert!(!sl1.search(10));
        sl1.add(10);
        assert!(sl1.search(10));

        assert!(!sl1.search(11));
        sl1.add(11);
        assert!(sl1.search(11));

        assert!(sl1.remove(11));
        assert!(!sl1.search(11));
        assert!(!sl1.remove(11));
    }
}


