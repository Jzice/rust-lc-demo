/*!
 * # [147.对链表进行插入排序](https://leetcode.cn/problems/insertion-sort-list/description/)
 *
 * @lc app=leetcode.cn id=147 lang=rust
 *
 * ## 难度
 * - Medium (69.50%)
 * - Likes:    600
 * - Dislikes: 0
 * - Total Accepted:    144K
 * - Total Submissions: 207.3K
 * - Testcase Example:  '[4,2,1,3]'
 *
 * ## 问题描述
 *
 * 给定单个链表的头 head ，使用 插入排序 对链表进行排序，并返回 排序后链表的头 。
 *
 * 插入排序 算法的步骤:
 *
 * - 插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的输出列表。
 * - 每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列中适当的位置，并将其插入。
 * - 重复直到所有输入数据插入完为止。
 *
 * 下面是插入排序算法的一个图形示例。部分排序的列表(黑色)最初只包含列表中的第一个元素。每次迭代时，从输入数据中删除一个元素(红色)，并就地插入已排序的列表中。
 *
 * 对链表进行插入排序。
 *
 *
 * ## 示例 1：
 * 输入: head = [4,2,1,3]
 * 输出: [1,2,3,4]
 *
 * ## 示例 2：
 * 输入: head = [-1,5,3,4,0]
 * 输出: [-1,0,3,4,5]
 *
 * ## 提示：
 * 列表中的节点数在 [1, 5000]范围内
 * -5000 <= Node.val <= 5000
 *
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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(node) => {
                let mut sorted = ListNode::new(std::i32::MIN); //已排序的list
                let mut unsorted = Some(node); //未排序的list

                // 如果存在未排序的节点
                while let Some(mut node_to_insert) = unsorted {
                    unsorted = node_to_insert.next.take(); //取下未排序链表头节点,
                                                           //保留后续未排序链表

                    let mut sorted_ref = &mut sorted; //设置一个可变指针指向已排序链表头
                    while sorted_ref.next.is_some()
                        && sorted_ref.next.as_ref().unwrap().val < node_to_insert.val
                    {
                        sorted_ref = sorted_ref.next.as_mut().unwrap();
                    }
                    //
                    node_to_insert.next = sorted_ref.next.take();
                    sorted_ref.next = Some(node_to_insert);
                }

                sorted.next
            }
        }
    }
}
// @lc code=end
