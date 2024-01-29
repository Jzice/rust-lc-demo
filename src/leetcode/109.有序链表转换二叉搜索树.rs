/*!
 * @lc app=leetcode.cn id=109 lang=rust
 *
 * # [109] 有序链表转换二叉搜索树
 *
 * https://leetcode.cn/problems/convert-sorted-list-to-binary-search-tree/description/
 *
 * algorithms
 * Medium (76.46%)
 * Likes:    848
 * Dislikes: 0
 * Total Accepted:    149K
 * Total Submissions: 194.9K
 * Testcase Example:  '[-10,-3,0,5,9]'
 *
 * 给定一个单链表的头节点  head ，其中的元素 按升序排序 ，将其转换为高度平衡的二叉搜索树。
 *
 * 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差不超过 1。
 *
 *
 *
 * 示例 1:
 *
 *
 *
 *
 * 输入: head = [-10,-3,0,5,9]
 * 输出: [0,-3,9,-10,null,5]
 * 解释: 一个可能的答案是[0，-3,9，-10,null,5]，它表示所示的高度平衡的二叉搜索树。
 *
 *
 * 示例 2:
 *
 *
 * 输入: head = []
 * 输出: []
 *
 *
 *
 *
 * 提示:
 *
 *
 * head 中的节点数在[0, 2 * 10^4] 范围内
 * -10^5 <= Node.val <= 10^5
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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// ## 解题思路
    /// - 递归
    /// 1. 先求出链表总长度;
    /// 2. 根据总长度, 将链表分为3个部分: 左半部分, 中间root节点, 右半部分;
    /// 3. 分别使用左半部分链表, 右半部分链表递归生成左子树,右子树;
    /// 4. 综合左右子树及中间root节点, 生成完整的二叉搜索树;
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }

        type ListNodeOpt = Option<Box<ListNode>>;
        type TreeNodeOpt = Option<Rc<RefCell<TreeNode>>>;

        /// 将list链表的前len个节点转换为二叉搜索树
        fn transfer_to_tree(head: &mut ListNodeOpt, len: usize) -> TreeNodeOpt {
            if len == 0 {
                return None;
            }

            // 先转换左侧len/2个节点为左子树
            let left = transfer_to_tree(head, len / 2);
            //
            if let Some(node) = head {
                // 得到当前节点的下一个节点, 并将head移动到该节点
                *head = node.next.take();

                // 将剩余链表转化为右子树
                let right = transfer_to_tree(head, len - len / 2 - 1);

                // 组装完整二叉搜索树
                Some(Rc::new(RefCell::new(TreeNode {
                    val: node.val,
                    left,
                    right,
                })))
            } else {
                None
            }
        }

        // 统计链表节点总数
        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }

        let mut head = head;
        transfer_to_tree(&mut head, len)
    }
}
// @lc code=end
