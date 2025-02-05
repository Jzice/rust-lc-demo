/*!
 * # [112.路径总和](https://leetcode.cn/problems/path-sum/description/)
 *
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * ## 难度
 * Easy (52.64%)
 * Likes:    709
 * Dislikes: 0
 * Total Accepted:    279.5K
 * Total Submissions: 531.1K
 * Testcase Example:  '[5,4,8,11,null,13,4,7,2,null,null,null,1]\n22'
 *
 * ## 描述
 *
 * 给你二叉树的根节点 root 和一个表示目标和的整数 targetSum ，判断该树中是否存在 根节点到叶子节点
 * 的路径，这条路径上所有节点值相加等于目标和 targetSum 。
 *
 * 叶子节点是指没有子节点的节点。
 *
 *
 * ## 示例 1：
 * - 输入：root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
 * - 输出：true
 *
 *
 * ## 示例 2：
 * - 输入：root = [1,2,3], targetSum = 5
 * - 输出：false
 *
 *
 * ## 示例 3：
 * - 输入：root = [1,2], targetSum = 0
 * - 输出：false
 *
 *
 * ## 提示：
 * - 树中节点的数目在范围 [0, 5000] 内
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
    /// ## 路径总和
    /// - 递归
    /// 1. 若节点为空，则遍历结束，未找到;
    /// 2. 若为叶子节点，且节点val==剩下的target, 则找到；
    /// 3. 否则递归在左右子树中查找target-node.val;
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        /// helper
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
            match node {
                None => false,
                Some(n)
                    if n.borrow().left.is_none()
                        && n.borrow().right.is_none()
                        && n.borrow().val == target_sum =>
                {
                    true
                }
                Some(n) => {
                    helper(&n.borrow().left, target_sum - n.borrow().val)
                        || helper(&n.borrow().right, target_sum - n.borrow().val)
                }
            }
        }

        helper(&root, target_sum)
    }
}
// @lc code=end
