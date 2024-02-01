/*!
 * # [98.验证二叉搜索树](https://leetcode.cn/problems/validate-binary-search-tree/description/)
 *
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * ## 难度
 *
 * - Medium (37.06%)
 * - Likes:    2111
 * - Dislikes: 0
 * - Total Accepted:    753.8K
 * - Total Submissions: 2M
 * - Testcase Example:  '[2,1,3]'
 *
 * ## 题目描述
 *
 * 给你一个二叉树的根节点 root ，判断其是否是一个有效的二叉搜索树。
 *
 * 有效 二叉搜索树定义如下：
 *
 * - 节点的左子树只包含 小于 当前节点的数。
 * - 节点的右子树只包含 大于 当前节点的数。
 * - 所有左子树和右子树自身必须也是二叉搜索树。
 *
 *
 *
 *
 * ## 示例 1：
 *
 * - 输入：root = [2,1,3]
 * - 输出：true
 *
 *
 * ## 示例 2：
 *
 * - 输入：root = [5,1,4,null,null,3,6]
 * - 输出：false
 * - 解释：根节点的值是 5 ，但是右子节点的值是 4 。
 *
 *
 * ## 提示：
 *
 * - 树中节点数目范围在[1, 10^4] 内
 * - -2^31 <= Node.val <= 2^31 - 1
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
    /// # 验证二叉搜索树
    /// ## 解题思路
    /// - 递归
    /// 1. bst合法的条件:
    ///    a. 空树合法;
    ///    b. 左子树为合法bst && 父节点值>左子树节点值.max();
    ///    c. 右子树为合法bst && 父节点值<右子树节点值.min();
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid(
            root: &Option<Rc<RefCell<TreeNode>>>,
            upper: Option<i32>,
            lower: Option<i32>,
        ) -> bool {
            root.as_ref().map_or(true, |root| {
                (lower.is_none() || root.borrow().val > lower.unwrap())
                    && (upper.is_none() || root.borrow().val < upper.unwrap())
                    && is_valid(&root.borrow().left, Some(root.borrow().val), lower.clone())
                    && is_valid(&root.borrow().right, upper.clone(), Some(root.borrow().val))
            })
        }

        is_valid(&root, None, None)
    }
}
// @lc code=end
