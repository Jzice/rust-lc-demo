/*!
 * # [21.合并两个有序链表]()
 * 
 * @lc app=leetcode.cn id=21 lang=rust
 * 
 * ## 难度
 * - Easy (66.22%)	3432
 * 
 * ## 问题描述
 * 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
 * 
 *  
 * 
 * ## 示例 1：
 * - 输入：l1 = [1,2,4], l2 = [1,3,4]
 * - 输出：[1,1,2,3,4,4]
 * 
 * ## 示例 2：
 * - 输入：l1 = [], l2 = []
 * - 输出：[]
 * 
 * ## 示例 3：
 * - 输入：l1 = [], l2 = [0]
 * - 输出：[0]
 * 
 * ## 提示：
 * - 两个链表的节点数目范围是 [0, 50]
 * - -100 <= Node.val <= 100
 * - l1 和 l2 均按 非递减顺序 排列
 */

use super::*;

// @lc code=start
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//    pub val: i32,
//    pub next: Option<Box<ListNode>>,
// }
//impl ListNode {
 //   #[inline]
 //  fn new(val: i32) -> Self {
 //       ListNode {
//            next: None,
//            val
//        }
//    }
//}
impl Solution {
    /// # 合并两个有序链表
    /// ## 解题思路
    /// - 递归
    /// 1. 同时递归遍历两个链表；
    /// 2. 遍历时, 比较两个链表结点值;
    ///   - 如果两个节点都为空，则为空；
    ///   - 如果有一个为空，则返回非空节点；
    ///   - 如果都不为空，则将小的节点取下，将剩下的和另一条链表递归合并；
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val < r.val {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}
// @lc code=end
