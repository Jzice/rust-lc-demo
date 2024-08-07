/*!
 * # [876.链表的中间结点](https://leetcode.cn/problems/middle-of-the-linked-list/description/)
 *
 * @lc app=leetcode.cn id=876 lang=rust
 *
 * ## 难度
 *
 * - Easy (63.45%)
 * - Likes:    144
 * - Dislikes: 0
 * - Total Accepted:    26.6K
 * - Total Submissions: 41.3K
 * - Testcase Example:  '[1,2,3,4,5]'
 *
 * ## 问题描述
 *
 * 给定一个带有头结点 head 的非空单链表，返回链表的中间结点。
 * 
 * 如果有两个中间结点，则返回第二个中间结点。
 * 
 * 
 * ## 示例 1：
 * 
 * - 输入：[1,2,3,4,5]
 * - 输出：此列表中的结点 3 (序列化形式：[3,4,5])
 * 返回的结点值为 3 。 (测评系统对该结点序列化表述是 [3,4,5])。
 * 注意，我们返回了一个 ListNode 类型的对象 ans，这样：
 * ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, 以及 ans.next.next.next
 * = NULL.
 * 
 * 
 * ## 示例 2：
 * 
 * - 输入：[1,2,3,4,5,6]
 * - 输出：此列表中的结点 4 (序列化形式：[4,5,6])
 * 由于该列表有两个中间结点，值分别为 3 和 4，我们返回第二个结点。
 * 
 * 
 * # 提示：
 * 
 * - 给定链表的结点数介于 1 和 100 之间。
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;  //move head to mut head
        let mut fast_ref = head.as_ref();  //share head fast ref
        let mut slow_ref = head.as_ref();  //share head slow ref

        // loop with fast and slow ref
        loop {

            if let Some(node) = fast_ref {
                fast_ref = node.next.as_ref();
            } else {
                break;
            }
            if let Some(node) = fast_ref {
                fast_ref = node.next.as_ref();
            } else {
                break;
            }
            if let Some(node) = slow_ref {
                slow_ref = node.next.as_ref();
            } else {
                break;
            }
        }

        // 记录中间节点
        let mid_ref = if let Some(node) = slow_ref {
            node.as_ref() as *const ListNode
        } else {
            return None;
        };
 
        //释放前半段链表
        while let Some(node) = head.as_ref() {
            let addr = node.as_ref() as *const ListNode;
            if addr != mid_ref {
                head = head.unwrap().next;
            } else {
                break;
            }
        }

        head
    }
}
// @lc code=end

