/*!
 * # [199.二叉树的右视图](https://leetcode.cn/problems/binary-tree-right-side-view/description/)
 *
 * @lc app=leetcode.cn id=199 lang=rust
 *
 * ## 难度
 * - Medium (65.76%)
 * - Likes:    827
 * - Dislikes: 0
 * - Total Accepted:    269.6K
 * - Total Submissions: 409.7K
 * - Testcase Example:  '[1,2,3,null,5,null,4]'
 *
 * ## 问题描述
 *
 * 给定一个二叉树的 根节点 root，想象自己站在它的右侧，按照从顶部到底部的顺序，返回从右侧所能看到的节点值。
 *
 * ## 示例 1:
 * - 输入: [1,2,3,null,5,null,4]
 * - 输出: [1,3,4]
 *
 * ## 示例 2:
 * - 输入: [1,null,3]
 * - 输出: [1,3]
 *
 * ## 示例 3:
 * - 输入: []
 * - 输出: []
 *
 * ## 提示:
 * - 二叉树的节点个数的范围是 [0,100]
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
    /// # 199.二叉树的右视图
    /// ## 解题思路
    /// - 队列
    /// 1. 层历，取每一层的最后一个元素
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut right_view_vals = Vec::new();
        let mut q = VecDeque::new();
        if let Some(root) = root {
            q.push_back(root.clone());
            while !q.is_empty() {
                let mut cur_vals = Vec::new();
                for _ in 0..q.len() {
                    q.pop_front().map(|node| {
                        cur_vals.push(node.borrow().val);
                        node.borrow().left.clone().map(|left| {
                            q.push_back(left);
                        });
                        node.borrow().right.clone().map(|right| {
                            q.push_back(right);
                        });
                    });
                }
                right_view_vals.push(*cur_vals.last().unwrap());
            }
        }

        right_view_vals
    }
}
// @lc code=end
