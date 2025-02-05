/*!
 * # [148.排序链表](https://leetcode.cn/problems/sort-list/description/)
 *
 * @lc app=leetcode.cn id=148 lang=rust
 *
 * ## 难度
 * - Medium (65.90%)
 * - Likes:    1948
 * - Dislikes: 0
 * - Total Accepted:    386.6K
 * - Total Submissions: 586.6K
 * - Testcase Example:  '[4,2,1,3]'
 *
 * ## 问题描述
 *
 * 给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。
 *
 *
 * ## 示例 1：
 * - 输入：head = [4,2,1,3]
 * - 输出：[1,2,3,4]
 *
 *
 * ## 示例 2：
 * - 输入：head = [-1,5,3,4,0]
 * - 输出：[-1,0,3,4,5]
 *
 * ## 示例 3：
 * - 输入：head = []
 * - 输出：[]
 *
 *
 * ## 提示：
 * - 链表中节点的数目在范围 [0, 5 * 10^4] 内
 * - -10^5 <= Node.val <= 10^5
 *
 * ## 进阶：
 * 你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序吗？
 *
 */

use super::*;

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    /// ## 排序链表
    /// 1. 从头至尾遍历链表,依次将各个节点值加入到数组vals中;
    /// 2. 排序vals数组;
    /// 3. 遍历排序后的vals数组,生成有序的新链表;
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals = Vec::new(); //记录个节点值

        let mut p = head.as_ref(); //设置遍历指针
        while let Some(node) = p {
            vals.push(node.val);
            p = node.next.as_ref();
        }

        vals.sort(); //对数组进行排序

        let mut next = None;
        while let Some(val) = vals.pop() {
            next = Some(Box::new(ListNode { next, val }));
        }

        next
    }
}
// @lc code=end
