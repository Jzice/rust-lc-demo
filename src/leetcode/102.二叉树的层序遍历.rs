/*!
 * # [102.二叉树的层次遍历](https://leetcode.cn/problems/binary-tree-level-order-traversal/description/)
 *
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * ## 难度
 * - Medium (60.88%)
 * - Likes:    391
 * - Dislikes: 0
 * - Total Accepted:    82.8K
 * - Total Submissions: 135.8K
 * - Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * ## 问题描述
 *
 * 给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。
 *
 * 给定二叉树: [3,9,20,null,null,15,7],
 *
 * ```text
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 * ```
 *
 * 返回其层次遍历结果：
 *
 * ```text
 * [
 * ⁠ [3],
 * ⁠ [9,20],
 * ⁠ [15,7]
 * ]
 * ```
 *
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    /// ## 解题思路
    /// - 队列
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn level_order_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut res = Vec::new(); //result
            let mut q = VecDeque::new();
            root.clone().map(|rc| {
                q.push_back(rc);
                while !q.is_empty() {
                    let mut vals = Vec::new();
                    for _ in 0..q.len() {
                        q.pop_front().map(|node| {
                            vals.push(node.borrow().val);
                            node.borrow().left.clone().map(|left| {
                                q.push_back(left);
                            });
                            node.borrow().right.clone().map(|right| {
                                q.push_back(right);
                            });
                        });
                    }

                    res.push(vals);
                }
            });

            res
        }
        level_order_ref(&root)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {

    #[test]
    fn test_level_order() {
        // assert_eq!(
        //     // Solution::level_order(build_tree(&vec![3, 9, 20, None, None, 15, 7])),
        //     // vec![vec![3], vec![9, 20], vec![15, 7],]
        // );
    }
}
