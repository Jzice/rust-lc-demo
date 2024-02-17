/*!
 * # [114.二叉树展开为链表](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/description/)
 *
 * @lc app=leetcode.cn id=114 lang=rust
 *
 * ## 难度
 * - Medium (73.02%)
 * - Likes:    1522
 * - Dislikes: 0
 * - Total Accepted:    374.9K
 * - Total Submissions: 513.2K
 * - Testcase Example:  '[1,2,5,3,4,null,6]'
 *
 * ## 描述
 *
 * 给你二叉树的根结点 root ，请你将它展开为一个单链表：
 *
 * - 展开后的单链表应该同样使用 TreeNode ，其中 right 子指针指向链表中下一个结点，而左子指针始终为 null 。
 * - 展开后的单链表应该与二叉树 先序遍历 顺序相同。
 *
 * ## 示例 1：
 * - 输入：root = [1,2,5,3,4,null,6]
 * - 输出：[1,null,2,null,3,null,4,null,5,null,6]
 *
 * ## 示例 2：
 * - 输入：root = []
 * - 输出：[]
 *
 * ## 示例 3：
 * - 输入：root = [0]
 * - 输出：[0]
 *
 * ## 提示：
 * - 树中结点数在范围 [0, 2000] 内
 * - -100
 *
 * ## 进阶：
 *
 * 你可以使用原地算法（O(1) 额外空间）展开这棵树吗？
 *
 */

use super::*;

// @lc code=start
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
    /// ## 二叉树展开为链表
    /// - 递归
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        match root {
            None => {}
            Some(node) => {
                // 取下当前节点左子树
                let mut left = node.borrow_mut().left.take();
                // 将左子树展开为链表
                Solution::flatten(&mut left);

                // 取下当前节点右子树
                let mut right = node.borrow_mut().right.take();
                // 将右子树展开为链表
                Solution::flatten(&mut right);

                // 将左子树展开后链表表头接到当前节点右指针后
                node.borrow_mut().right = left;

                // 遍历已接左子树链表, 找到最后一个节点
                let mut p = node.clone();
                while p.borrow().right.is_some() {
                    let tmp = p.borrow().right.clone().unwrap();
                    p = tmp;
                }
                // 将右子树链表头接到最后一个节点右子树上
                p.borrow_mut().right = right;
            }
        }
    }
}
// @lc code=end
