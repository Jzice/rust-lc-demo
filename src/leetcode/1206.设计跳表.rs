/*!
 * # [1206.设计跳表](https://leetcode.cn/problems/design-skiplist/description/)
 *
 * @lc app=leetcode.cn id=1206 lang=rust
 *
 * ## 难度
 * - Hard (68.88%)
 * - Likes:    253
 * - Dislikes: 0
 * - Total Accepted:    28.5K
 * - Total Submissions: 41.5K
 * - Testcase Example:  '["Skiplist","add","add","add","search","add","search","erase","erase","search"]\n' +
 * '[[],[1],[2],[3],[0],[4],[1],[0],[1],[1]]'
 *
 * ## 问题描述
 *
 * 不使用任何库函数，设计一个 跳表 。
 *
 * 跳表 是在 O(log(n)) 时间内完成增加、删除、搜索操作的数据结构。跳表相比于树堆与红黑树，其功能与性能相当，并且跳表的代码长度相较下更短，其设计思想与链表相似。
 *
 * 例如，一个跳表包含 [30, 40, 50, 60, 70, 90] ，然后增加 80、45 到跳表中，以下图的方式操作：
 *
 *
 * Artyom Kalinin [CC BY-SA 3.0], via Wikimedia Commons
 *
 * 跳表中有很多层，每一层是一个短的链表。在第一层的作用下，增加、删除和搜索操作的时间复杂度不超过 O(n)。跳表的每一个操作的平均时间复杂度是
 * O(log(n))，空间复杂度是 O(n)。
 *
 * 了解更多 : [wiki](https://en.wikipedia.org/wiki/Skip_list)
 *
 * 在本题中，你的设计应该要包含这些函数：
 * ```cpp
 * bool search(int target);  //返回target是否存在于跳表中。
 * void add(int num);        // 插入一个元素到跳表。
 * bool erase(int num); // 在跳表中删除一个值, 如果num不存在,直接返回false. 如果存在多个num, 删除其中任意一个即可。
 * ```
 *
 * 注意，跳表中可能存在多个相同的值，你的代码需要处理这种情况。
 *
 * ## 示例 1:
 * - 输入
 * ["Skiplist", "add", "add", "add", "search", "add", "search", "erase",
 * "erase", "search"]
 * [[], [1], [2], [3], [0], [4], [1], [0], [1], [1]]
 * - 输出
 * [null, null, null, null, false, null, true, false, true, false]
 *
 * - 解释
 * ```cpp
 * Skiplist skiplist = new Skiplist();
 * skiplist.add(1);
 * skiplist.add(2);
 * skiplist.add(3);
 * skiplist.search(0);   // 返回 false
 * skiplist.add(4);
 * skiplist.search(1);   // 返回 true
 * skiplist.erase(0);    // 返回 false，0 不在跳表中
 * skiplist.erase(1);    // 返回 true
 * skiplist.search(1);   // 返回 false，1 已被擦除
 * ```
 *
 * ## 提示:
 * - 0 <= num, target <= 2 * 10^4
 * - 调用search, add,  erase操作次数不大于 5 * 10^4
 *
 */

// @lc code=start
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const P_FACTOR: f64 = 0.25; // 负载因子
const LEVEL_MAX: usize = 32; // 最大层级

// 跳表节点间的连接
type Link = Option<Rc<RefCell<SkipNode>>>;

// 跳表节点
struct SkipNode {
    val: i32,        // 节点值
    next: Vec<Link>, // 下一个
}

impl SkipNode {
    fn new(level: usize, val: i32) -> Self {
        Self {
            val,
            next: vec![None; level],
        }
    }
}

// 跳表
struct Skiplist {
    level: usize,
    head: Link, //跳表头
}

// 生成随机level
fn random_level() -> usize {
    let mut level = 1;
    let mut rng = rand::thread_rng();
    while level < LEVEL_MAX && rng.gen::<f64>() < P_FACTOR {
        level += 1;
    }
    level
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    /// 新建跳表
    pub fn new() -> Self {
        Self {
            level: LEVEL_MAX,
            head: Some(Rc::new(RefCell::new(SkipNode::new(
                LEVEL_MAX,
                std::i32::MIN,
            )))),
        }
    }

    /// 查找元素是否存在
    pub fn search(&self, target: i32) -> bool {
        let mut node_opt = self.head.clone(); //从表头开始

        // 从上往下遍历各层
        for l in (0..self.level).rev() {
            // 从左往右遍历当前层的各节点
            while let Some(node) = node_opt.clone() {
                // 当前节点值=目标节点
                if node.borrow().val == target {
                    return true; //找到, 返回true
                } else if let Some(next) = node.borrow().next[l]
                    .clone()
                    .filter(|n| n.borrow().val <= target)
                {
                    //否则, 下个节点值 <= 目标值
                    node_opt.replace(next); //改变当前遍历指针到右边下一个节点
                } else {
                    //否则下个节点值>目标值,或下个节点为空
                    break; //结束当前层的遍历, 从下一层当前节点处继续遍历
                }
            }
        }

        // 没有找到与目标值相等的节点
        return false;
    }

    // 插入数据
    pub fn add(&mut self, num: i32) {
        let level = random_level();
        let new_node = Rc::new(RefCell::new(SkipNode::new(level, num)));
        let mut link_opt = self.head.clone();
        // 从新节点层级开始,自上往下遍历各层
        for l in (0..level).rev() {
            // 从左往右遍历当前层的各个节点
            while let Some(link) = link_opt.clone() {
                let mut node = link.borrow_mut();
                if node.val == num {
                    return;
                }

                // 如果下个节点存在,且值<新节点值
                if let Some(next) = node.next[l].clone().filter(|n| n.borrow().val < num) {
                    link_opt.replace(next.clone()); //将当前节点替换为下个节点
                } else {
                    //否则下个节点不存在,或者节点值>=新节点值

                    // 如果下个节点存在,且节点值>=新节点值
                    /*
                    if let Some(next) = node.next[l].clone().filter(|n| n.borrow().val >= num) {
                    }
                     */
                    new_node.borrow_mut().next[l] = node.next[l].take(); //将新节点的该level下个指针指向下个节点;

                    // 将当前节点的该级指针指向新节点
                    node.next[l].replace(new_node.clone());
                    break; //结束当前级遍历
                }
            }
        }
    }

    /// 删除对于值的元素
    /// erase node which val == num
    /// when node exist, remove node and return true
    /// else return false
    fn erase(&mut self, num: i32) -> bool {
        let mut existed = false;

        let mut link_opt = self.head.clone();
        // let level = self.level - 1;
        for l in (0..self.level).rev() {
            //从上至下,依次遍历各层
            while let Some(link) = link_opt.clone() {
                let mut node = link.borrow_mut();
                // 如果下个节点的值<=目标值
                if let Some(next) = node.next[l].clone().filter(|n| n.borrow().val <= num) {
                    let next_val = next.borrow().val;
                    if next_val < num {
                        //下个节点值<目标节点, 向右移动指针
                        link_opt.replace(next.clone());
                    } else if next_val == num {
                        //下个节点值==需要删除的节点
                        node.next[l] = next.clone().borrow_mut().next[l].take(); //移除需要被删除的节点
                        existed = true; //存在需要被删除的节点
                        break;
                    } else {
                        break;
                    }
                } else {
                    //下个节点值>目标值或不存在下个节点
                    break; //退出当前层遍历,进入下一层遍历
                }
            }
        }

        existed
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut sl = Skiplist::new();
        sl.add(1);
        sl.add(2);
        sl.add(3);
        assert!(sl.search(0) == false);
        sl.add(4);
        assert!(sl.search(1) == true);
        assert!(sl.erase(0) == false);
        assert!(sl.erase(1) == true);
        assert!(sl.search(1) == false);
    }
}
