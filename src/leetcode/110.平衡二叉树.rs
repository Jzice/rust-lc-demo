/*!
 * @lc app=leetcode.cn id=110 lang=rust
 *
 * # [110] 平衡二叉树
 *
 * https://leetcode.cn/problems/balanced-binary-tree/description/
 *
 * algorithms
 * Easy (57.55%)
 * Likes:    1371
 * Dislikes: 0
 * Total Accepted:    524.7K
 * Total Submissions: 911.2K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给定一个二叉树，判断它是否是高度平衡的二叉树。
 *
 * 本题中，一棵高度平衡二叉树定义为：
 *
 *
 * 一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1 。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：root = [3,9,20,null,null,15,7]
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：root = [1,2,2,3,3,null,null,4,4]
 * 输出：false
 *
 *
 * 示例 3：
 *
 *
 * 输入：root = []
 * 输出：true
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中的节点数在范围 [0, 5000] 内
 * -10^4
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
use std::rc::Rc;
impl Solution {
    /// ## 解题思路
    /// - 递归
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type TreeNodeOpt = Option<Rc<RefCell<TreeNode>>>;
        fn get_height(root: &TreeNodeOpt) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    get_height(&node.borrow().left).max(get_height(&node.borrow().right)) + 1
                }
            }
        }

        match &root {
            None => true,
            Some(root) => {
                Solution::is_balanced(root.borrow().left.clone())
                    && Solution::is_balanced(root.borrow().right.clone())
                    && (get_height(&root.borrow().left) - get_height(&root.borrow().right)).abs()
                        <= 1
            }
        }
    }
}
// @lc code=end
