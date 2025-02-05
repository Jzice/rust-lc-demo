/*!
 * # [1669.合并两个链表](https://leetcode.cn/problems/merge-in-between-linked-lists/description/)
 *
 * @lc app=leetcode.cn id=1669 lang=rust
 *
 * ## 难度
 * - Medium (77.41%)
 * - Likes:    95
 * - Dislikes: 0
 * - Total Accepted:    43.7K
 * - Total Submissions: 56.5K
 * - Testcase Example:  '[0,1,2,3,4,5]\n3\n4\n[1000000,1000001,1000002]'
 *
 * ## 问题描述
 *
 * 给你两个链表 list1 和 list2 ，它们包含的元素分别为 n 个和 m 个。
 *
 * 请你将 list1 中下标从 a 到 b 的全部节点都删除，并将list2 接在被删除节点的位置。
 *
 * 下图中蓝色边和节点展示了操作后的结果：
 *
 * 请你返回结果链表的头指针。
 *
 * ## 示例 1：
 * - 输入：list1 = [0,1,2,3,4,5], a = 3, b = 4, list2 = [1000000,1000001,1000002]
 * - 输出：[0,1,2,1000000,1000001,1000002,5]
 * - 解释：我们删除 list1 中下标为 3 和 4 的两个节点，并将 list2 接在该位置。上图中蓝色的边和节点为答案链表。
 *
 * ## 示例 2：
 * - 输入：list1 = [0,1,2,3,4,5,6], a = 2, b = 5, list2 = [1000000,1000001,1000002,1000003,1000004]
 * - 输出：[0,1,1000000,1000001,1000002,1000003,1000004,6]
 * - 解释：上图中蓝色的边和节点为答案链表。
 *
 * ## 提示：
 * - 3 <= list1.length <= 10^4
 * - 1 <= a <= b < list1.length - 1
 * - 1 <= list2.length <= 10^4
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
    /// 合并两个链表
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = list1.unwrap(); //头节点指针
        let mut current = head.as_mut();

        // 将节点遍历指针current跳过前面a个节点
        for _ in 0..(a - 1) {
            current = current.next.as_mut().unwrap();
        }

        // 交换current.next和list2,
        // current.next指向原list2, 原list2指向a节点
        std::mem::swap(&mut current.next, &mut list2);

        // 释放a->b之间的节点
        for _ in 0..(b - a + 1) {
            list2 = list2.unwrap().next;
        }

        // 移动current到尾部
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        //将b之后的节点连接到current.next上
        current.next = list2;

        Some(head)
    }
}
// @lc code=end
