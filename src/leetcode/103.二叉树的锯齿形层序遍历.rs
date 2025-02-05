/*!
 * # [103.二叉树的锯齿形层序遍历](https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/description/)
 *
 * @lc app=leetcode.cn id=103 lang=rust
 *
 * ## 难度
 * - Medium (57.54%)
 * - Likes:    752
 * - Dislikes: 0
 * - Total Accepted:    293.9K
 * - Total Submissions: 510.8K
 * - Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * ## 问题描述
 * 给你二叉树的根节点 root ，返回其节点值的 锯齿形层序遍历 。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。
 *
 * ## 示例 1：
 * - 输入：root = [3,9,20,null,null,15,7]
 * - 输出：[[3],[20,9],[15,7]]
 *
 * ## 示例 2：
 * - 输入：root = [1]
 * - 输出：[[1]]
 *
 * ## 示例 3：
 * - 输入：root = []
 * - 输出：[]
 *
 * ## 提示：
 * - 树中节点数目在范围 [0, 2000] 内
 * - -100 <= Node.val <= 100
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
    /// # 二叉树的锯齿形层序遍历
    /// ## 解题思路
    /// - 队列
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        let mut reverse_dir = true;
        let mut queue = Vec::new();
        let mut tmp = Vec::new();
        queue.push(root.clone().unwrap());
        while !queue.is_empty() {
            let mut vals = Vec::new();
            while let Some(node) = queue.pop() {
                let node = node.borrow();
                vals.push(node.val);
                match reverse_dir {
                    true => {
                        node.left.clone().map(|n| tmp.push(n));
                        node.right.clone().map(|n| tmp.push(n));
                    }
                    _ => {
                        node.right.clone().map(|n| tmp.push(n));
                        node.left.clone().map(|n| tmp.push(n));
                    }
                }
            }
            reverse_dir = !reverse_dir;

            res.push(vals);

            std::mem::swap(&mut queue, &mut tmp);
        }
        res
    }
}
// @lc code=end
